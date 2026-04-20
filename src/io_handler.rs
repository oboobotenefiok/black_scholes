use std::io;

pub fn get_f64(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!(" Please enter a valid number.\n"),
        }
    }
}

pub fn get_all_inputs() -> (f64, f64, f64, f64, f64) /*There should be a shorter way to write these repeated types like in Arrays :-) */ {
    println!("\n Black-Scholes European Call Option Pricer");
   
    
    let s = get_f64("Spot price (S): ");
    let k = get_f64("Strike price (K): ");
    let t = get_f64("Time to expiry in years (T): ");
    let r = get_f64("Risk-free rate (r) [e.g., 0.05 for 5%]: ");
    let sigma = get_f64("Volatility (σ) [e.g., 0.2 for 20%]: ");
    
    (s, k, t, r, sigma)
}