use rand::thread_rng;
use rand_distr::{Distribution, Normal};
use crate::black_scholes::OptionType;

/// Monte Carlo option pricing simulator
pub struct MonteCarloSimulator {
    spot_price: f64,
    strike_price: f64,
    time_to_expiry: f64,
    risk_free_rate: f64,
    volatility: f64,
    num_simulations: usize,
    option_type: OptionType,
}

impl MonteCarloSimulator {
    /// Create a new Monte Carlo simulator
    pub fn new(
        spot_price: f64,
        strike_price: f64,
        time_to_expiry: f64,
        risk_free_rate: f64,
        volatility: f64,
        num_simulations: usize,
        option_type: OptionType,
    ) -> Self {
        Self {
            spot_price,
            strike_price,
            time_to_expiry,
            risk_free_rate,
            volatility,
            num_simulations,
            option_type,
        }
    }

    /// Simulate final stock price using Geometric Brownian Motion
    fn simulate_price(&self, z: f64) -> f64 {
        let drift = (self.risk_free_rate - 0.5 * self.volatility.powi(2)) * self.time_to_expiry;
        let diffusion = self.volatility * self.time_to_expiry.sqrt() * z;
        self.spot_price * (drift + diffusion).exp()
    }

    /// Calculate option payoff
    fn payoff(&self, final_price: f64) -> f64 {
        match self.option_type {
            OptionType::Call => (final_price - self.strike_price).max(0.0),
            OptionType::Put => (self.strike_price - final_price).max(0.0),
        }
    }

    /// Price option using Monte Carlo simulation
    pub fn price(&self) -> f64 {
        let mut rng = thread_rng();
        let normal = Normal::new(0.0, 1.0).unwrap();

        let sum_payoffs: f64 = (0..self.num_simulations)
            .map(|_| {
                let z = normal.sample(&mut rng);
                let final_price = self.simulate_price(z);
                self.payoff(final_price)
            })
            .sum();

        let average_payoff = sum_payoffs / self.num_simulations as f64;
        average_payoff * (-self.risk_free_rate * self.time_to_expiry).exp()
    }

    /// Price option with confidence interval
    pub fn price_with_confidence(&self) -> (f64, f64, f64) {
        let mut rng = thread_rng();
        let normal = Normal::new(0.0, 1.0).unwrap();

        let payoffs: Vec<f64> = (0..self.num_simulations)
            .map(|_| {
                let z = normal.sample(&mut rng);
                let final_price = self.simulate_price(z);
                self.payoff(final_price)
            })
            .collect();

        let mean = payoffs.iter().sum::<f64>() / self.num_simulations as f64;
        let variance = payoffs
            .iter()
            .map(|p| (p - mean).powi(2))
            .sum::<f64>()
            / (self.num_simulations - 1) as f64;
        let std_error = variance.sqrt() / (self.num_simulations as f64).sqrt();

        let discount_factor = (-self.risk_free_rate * self.time_to_expiry).exp();
        let price = mean * discount_factor;
        let confidence_interval = 1.96 * std_error * discount_factor; // 95% CI

        (price, price - confidence_interval, price + confidence_interval)
    }

    /// Calculate option delta using finite difference
    pub fn delta(&self) -> f64 {
        let epsilon = 0.01 * self.spot_price;

        let mut sim_up = self.clone();
        sim_up.spot_price += epsilon;

        let mut sim_down = self.clone();
        sim_down.spot_price -= epsilon;

        (sim_up.price() - sim_down.price()) / (2.0 * epsilon)
    }

    /// Calculate option gamma using finite difference
    pub fn gamma(&self) -> f64 {
        let epsilon = 0.01 * self.spot_price;

        let mut sim_up = self.clone();
        sim_up.spot_price += epsilon;

        let mut sim_down = self.clone();
        sim_down.spot_price -= epsilon;

        let price_center = self.price();
        let price_up = sim_up.price();
        let price_down = sim_down.price();

        (price_up - 2.0 * price_center + price_down) / epsilon.powi(2)
    }
}

impl Clone for MonteCarloSimulator {
    fn clone(&self) -> Self {
        Self {
            spot_price: self.spot_price,
            strike_price: self.strike_price,
            time_to_expiry: self.time_to_expiry,
            risk_free_rate: self.risk_free_rate,
            volatility: self.volatility,
            num_simulations: self.num_simulations,
            option_type: self.option_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monte_carlo_call_price() {
        let mc = MonteCarloSimulator::new(
            100.0,
            100.0,
            1.0,
            0.05,
            0.2,
            10000,
            OptionType::Call,
        );

        let price = mc.price();
        assert!(price > 0.0);
        assert!(price < 100.0);
    }

    #[test]
    fn test_monte_carlo_put_price() {
        let mc = MonteCarloSimulator::new(
            100.0,
            100.0,
            1.0,
            0.05,
            0.2,
            10000,
            OptionType::Put,
        );

        let price = mc.price();
        assert!(price > 0.0);
        assert!(price < 100.0);
    }

    #[test]
    fn test_confidence_interval() {
        let mc = MonteCarloSimulator::new(
            100.0,
            100.0,
            1.0,
            0.05,
            0.2,
            10000,
            OptionType::Call,
        );

        let (price, lower, upper) = mc.price_with_confidence();
        
        assert!(lower < price);
        assert!(price < upper);
        assert!(lower > 0.0);
    }

    #[test]
    fn test_delta_range() {
        let mc = MonteCarloSimulator::new(
            100.0,
            100.0,
            1.0,
            0.05,
            0.2,
            5000,
            OptionType::Call,
        );

        let delta = mc.delta();
        assert!(delta >= 0.0 && delta <= 1.0);
    }
}
