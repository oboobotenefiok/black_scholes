mod io_handler;
mod black_scholes;

fn main() {
    let (s, k, t, r, sigma) = io_handler::get_all_inputs();
    
    let call = black_scholes::call_price(s, k, t, r, sigma);
    let put = black_scholes::put_price(s, k, t, r, sigma);
    let delta = black_scholes::delta_call(s, k, t, r, sigma);
    
    println!("\n Results:");
    println!("   Call option price: {:.4}", call);
    println!("   Put option price:  {:.4}", put);
    println!("   Delta (call):      {:.4}", delta);
   
}