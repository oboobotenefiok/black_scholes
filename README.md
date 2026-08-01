#### Black-Scholes Option Pricer

A simple Rust implementation of the Black-Scholes model for pricing European call and put options, including delta calculation.

#### Features

- Calculate European call option prices
- Calculate European put option prices (via put-call parity)
- Calculate call option delta
- Interactive command-line input
- Handles edge cases (zero time to expiry)

#### Usage

Clone the repo, and in the project root:

```bash
cargo run
```

Then enter the required inputs when prompted:

- Spot price (S)
- Strike price (K)
- Time to expiry in years (T)
- Risk-free rate (r) - e.g., 0.05 for 5%
- Volatility (σ) - e.g., 0.2 for 20%

#### Project Structure

- main.rs - Entry point
- black_scholes.rs - Pricing models and calculations
- io_handler.rs - Input handling

#### Dependencies

- Rust standard library only

With Love,

- Obot
