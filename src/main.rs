use rust_options_pricing::{BlackScholes, MonteCarloSimulator, OptionType};

fn main() {
    println!("=== Rust Options Pricing Engine ===\n");

    // Example parameters
    let spot_price = 100.0;
    let strike_price = 100.0;
    let time_to_expiry = 1.0; // 1 year
    let risk_free_rate = 0.05; // 5%
    let volatility = 0.25; // 25%

    // Black-Scholes pricing
    println!("--- Black-Scholes Model ---");
    
    let call = BlackScholes::new(
        spot_price,
        strike_price,
        time_to_expiry,
        risk_free_rate,
        volatility,
        OptionType::Call,
    );

    let put = BlackScholes::new(
        spot_price,
        strike_price,
        time_to_expiry,
        risk_free_rate,
        volatility,
        OptionType::Put,
    );

    println!("Call Option:");
    println!("  Price: ${:.4}", call.price());
    println!("  Delta: {:.4}", call.delta());
    println!("  Gamma: {:.4}", call.gamma());
    println!("  Vega: {:.4}", call.vega());
    println!("  Theta: {:.4}", call.theta());
    println!("  Rho: {:.4}", call.rho());

    println!("\nPut Option:");
    println!("  Price: ${:.4}", put.price());
    println!("  Delta: {:.4}", put.delta());
    println!("  Gamma: {:.4}", put.gamma());
    println!("  Vega: {:.4}", put.vega());
    println!("  Theta: {:.4}", put.theta());
    println!("  Rho: {:.4}", put.rho());

    // Implied Volatility
    println!("\n--- Implied Volatility ---");
    let market_price = 12.5;
    if let Some(iv) = BlackScholes::implied_volatility(
        spot_price,
        strike_price,
        time_to_expiry,
        risk_free_rate,
        market_price,
        OptionType::Call,
    ) {
        println!("Market Price: ${:.2}", market_price);
        println!("Implied Volatility: {:.2}%", iv * 100.0);
    }

    // Monte Carlo simulation
    println!("\n--- Monte Carlo Simulation ---");
    
    let mc_call = MonteCarloSimulator::new(
        spot_price,
        strike_price,
        time_to_expiry,
        risk_free_rate,
        volatility,
        100000,
        OptionType::Call,
    );

    let (price, lower, upper) = mc_call.price_with_confidence();
    println!("Call Option (100,000 simulations):");
    println!("  Price: ${:.4}", price);
    println!("  95% CI: [${:.4}, ${:.4}]", lower, upper);
    println!("  Delta: {:.4}", mc_call.delta());

    // Comparison
    println!("\n--- Model Comparison ---");
    println!("Black-Scholes Call: ${:.4}", call.price());
    println!("Monte Carlo Call:   ${:.4}", price);
    println!("Difference:         ${:.4}", (call.price() - price).abs());

    println!("\n=== Demo completed ===");
}
