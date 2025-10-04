use statrs::distribution::{ContinuousCDF, Normal};

/// Option type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptionType {
    Call,
    Put,
}

/// Black-Scholes option pricing model
pub struct BlackScholes {
    spot_price: f64,
    strike_price: f64,
    time_to_expiry: f64,
    risk_free_rate: f64,
    volatility: f64,
    option_type: OptionType,
}

impl BlackScholes {
    /// Create a new Black-Scholes calculator
    pub fn new(
        spot_price: f64,
        strike_price: f64,
        time_to_expiry: f64,
        risk_free_rate: f64,
        volatility: f64,
        option_type: OptionType,
    ) -> Self {
        Self {
            spot_price,
            strike_price,
            time_to_expiry,
            risk_free_rate,
            volatility,
            option_type,
        }
    }

    /// Calculate d1 parameter
    fn d1(&self) -> f64 {
        let numerator = (self.spot_price / self.strike_price).ln()
            + (self.risk_free_rate + 0.5 * self.volatility.powi(2)) * self.time_to_expiry;
        let denominator = self.volatility * self.time_to_expiry.sqrt();
        numerator / denominator
    }

    /// Calculate d2 parameter
    fn d2(&self) -> f64 {
        self.d1() - self.volatility * self.time_to_expiry.sqrt()
    }

    /// Calculate option price
    pub fn price(&self) -> f64 {
        let normal = Normal::new(0.0, 1.0).unwrap();
        let d1 = self.d1();
        let d2 = self.d2();

        match self.option_type {
            OptionType::Call => {
                self.spot_price * normal.cdf(d1)
                    - self.strike_price
                        * (-self.risk_free_rate * self.time_to_expiry).exp()
                        * normal.cdf(d2)
            }
            OptionType::Put => {
                self.strike_price * (-self.risk_free_rate * self.time_to_expiry).exp()
                    * normal.cdf(-d2)
                    - self.spot_price * normal.cdf(-d1)
            }
        }
    }

    /// Calculate Delta (rate of change of option price with respect to underlying price)
    pub fn delta(&self) -> f64 {
        let normal = Normal::new(0.0, 1.0).unwrap();
        let d1 = self.d1();

        match self.option_type {
            OptionType::Call => normal.cdf(d1),
            OptionType::Put => normal.cdf(d1) - 1.0,
        }
    }

    /// Calculate Gamma (rate of change of Delta with respect to underlying price)
    pub fn gamma(&self) -> f64 {
        let normal = Normal::new(0.0, 1.0).unwrap();
        let d1 = self.d1();
        let pdf = (-0.5 * d1.powi(2)).exp() / (2.0 * std::f64::consts::PI).sqrt();

        pdf / (self.spot_price * self.volatility * self.time_to_expiry.sqrt())
    }

    /// Calculate Vega (sensitivity to volatility)
    pub fn vega(&self) -> f64 {
        let normal = Normal::new(0.0, 1.0).unwrap();
        let d1 = self.d1();
        let pdf = (-0.5 * d1.powi(2)).exp() / (2.0 * std::f64::consts::PI).sqrt();

        self.spot_price * pdf * self.time_to_expiry.sqrt() / 100.0
    }

    /// Calculate Theta (time decay)
    pub fn theta(&self) -> f64 {
        let normal = Normal::new(0.0, 1.0).unwrap();
        let d1 = self.d1();
        let d2 = self.d2();
        let pdf = (-0.5 * d1.powi(2)).exp() / (2.0 * std::f64::consts::PI).sqrt();

        let term1 = -(self.spot_price * pdf * self.volatility)
            / (2.0 * self.time_to_expiry.sqrt());

        match self.option_type {
            OptionType::Call => {
                let term2 = self.risk_free_rate
                    * self.strike_price
                    * (-self.risk_free_rate * self.time_to_expiry).exp()
                    * normal.cdf(d2);
                (term1 - term2) / 365.0
            }
            OptionType::Put => {
                let term2 = self.risk_free_rate
                    * self.strike_price
                    * (-self.risk_free_rate * self.time_to_expiry).exp()
                    * normal.cdf(-d2);
                (term1 + term2) / 365.0
            }
        }
    }

    /// Calculate Rho (sensitivity to interest rate)
    pub fn rho(&self) -> f64 {
        let normal = Normal::new(0.0, 1.0).unwrap();
        let d2 = self.d2();

        match self.option_type {
            OptionType::Call => {
                self.strike_price
                    * self.time_to_expiry
                    * (-self.risk_free_rate * self.time_to_expiry).exp()
                    * normal.cdf(d2)
                    / 100.0
            }
            OptionType::Put => {
                -self.strike_price
                    * self.time_to_expiry
                    * (-self.risk_free_rate * self.time_to_expiry).exp()
                    * normal.cdf(-d2)
                    / 100.0
            }
        }
    }

    /// Calculate all Greeks at once
    pub fn greeks(&self) -> Greeks {
        Greeks {
            delta: self.delta(),
            gamma: self.gamma(),
            vega: self.vega(),
            theta: self.theta(),
            rho: self.rho(),
        }
    }

    /// Calculate implied volatility using Newton-Raphson method
    pub fn implied_volatility(
        spot_price: f64,
        strike_price: f64,
        time_to_expiry: f64,
        risk_free_rate: f64,
        market_price: f64,
        option_type: OptionType,
    ) -> Option<f64> {
        let mut volatility = 0.5; // Initial guess
        let tolerance = 1e-6;
        let max_iterations = 100;

        for _ in 0..max_iterations {
            let bs = BlackScholes::new(
                spot_price,
                strike_price,
                time_to_expiry,
                risk_free_rate,
                volatility,
                option_type,
            );

            let price = bs.price();
            let vega = bs.vega() * 100.0; // Convert back to percentage

            let diff = market_price - price;

            if diff.abs() < tolerance {
                return Some(volatility);
            }

            if vega.abs() < 1e-10 {
                return None; // Avoid division by zero
            }

            volatility += diff / vega;

            if volatility <= 0.0 || volatility > 5.0 {
                return None; // Invalid volatility
            }
        }

        None // Did not converge
    }
}

/// Greeks container
#[derive(Debug, Clone, Copy)]
pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub vega: f64,
    pub theta: f64,
    pub rho: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_call_option_price() {
        let bs = BlackScholes::new(100.0, 100.0, 1.0, 0.05, 0.2, OptionType::Call);
        let price = bs.price();
        assert!(price > 0.0);
        assert!(price < 100.0);
    }

    #[test]
    fn test_put_option_price() {
        let bs = BlackScholes::new(100.0, 100.0, 1.0, 0.05, 0.2, OptionType::Put);
        let price = bs.price();
        assert!(price > 0.0);
        assert!(price < 100.0);
    }

    #[test]
    fn test_put_call_parity() {
        let spot = 100.0;
        let strike = 100.0;
        let time = 1.0;
        let rate = 0.05;
        let vol = 0.2;

        let call = BlackScholes::new(spot, strike, time, rate, vol, OptionType::Call);
        let put = BlackScholes::new(spot, strike, time, rate, vol, OptionType::Put);

        let call_price = call.price();
        let put_price = put.price();

        // Put-Call Parity: C - P = S - K * e^(-rT)
        let lhs = call_price - put_price;
        let rhs = spot - strike * (-rate * time).exp();

        assert_relative_eq!(lhs, rhs, epsilon = 1e-6);
    }

    #[test]
    fn test_delta_range() {
        let bs = BlackScholes::new(100.0, 100.0, 1.0, 0.05, 0.2, OptionType::Call);
        let delta = bs.delta();
        assert!(delta >= 0.0 && delta <= 1.0);

        let bs_put = BlackScholes::new(100.0, 100.0, 1.0, 0.05, 0.2, OptionType::Put);
        let delta_put = bs_put.delta();
        assert!(delta_put >= -1.0 && delta_put <= 0.0);
    }

    #[test]
    fn test_gamma_positive() {
        let bs = BlackScholes::new(100.0, 100.0, 1.0, 0.05, 0.2, OptionType::Call);
        let gamma = bs.gamma();
        assert!(gamma > 0.0);
    }

    #[test]
    fn test_vega_positive() {
        let bs = BlackScholes::new(100.0, 100.0, 1.0, 0.05, 0.2, OptionType::Call);
        let vega = bs.vega();
        assert!(vega > 0.0);
    }

    #[test]
    fn test_implied_volatility() {
        let spot = 100.0;
        let strike = 100.0;
        let time = 1.0;
        let rate = 0.05;
        let vol = 0.25;

        let bs = BlackScholes::new(spot, strike, time, rate, vol, OptionType::Call);
        let market_price = bs.price();

        let implied_vol = BlackScholes::implied_volatility(
            spot,
            strike,
            time,
            rate,
            market_price,
            OptionType::Call,
        );

        assert!(implied_vol.is_some());
        assert_relative_eq!(implied_vol.unwrap(), vol, epsilon = 1e-4);
    }
}
