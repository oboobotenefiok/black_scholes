use std::f64::consts::PI;

//I love this project so much!
pub fn call_price(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> f64 {
    if t <= 0.0 {
        return (s - k).max(0.0);
    }
    
    let d1 = calculate_d1(s, k, t, r, sigma);
    let d2 = d1 - sigma * t.sqrt();
    
    s * norm_cdf(d1) - k * (-r * t).exp() * norm_cdf(d2)
}

pub fn put_price(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> f64 {
    // Put-call parity: P = C - S + K*e^(-rT)
    let call = call_price(s, k, t, r, sigma);
    call - s + k * (-r * t).exp()
}

pub fn delta_call(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> f64 {
    if t <= 0.0 {
        return if s > k { 1.0 } else { 0.0 };
    }
    let d1 = calculate_d1(s, k, t, r, sigma);
    norm_cdf(d1)
}

fn calculate_d1(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> f64 {
    ((s / k).ln() + (r + 0.5 * sigma.powi(2)) * t) / (sigma * t.sqrt())
}

fn norm_cdf(x: f64) -> f64 {
    let k = 1.0 / (1.0 + 0.2316419 * x.abs());
    let a1 = 0.31938153;
    let a2 = -0.356563782;
    let a3 = 1.781477937;
    let a4 = -1.821255978;
    let a5 = 1.330274429;
    
    let pdf = (-x.powi(2) / 2.0).exp() / (2.0 * PI).sqrt();
    let phi = 1.0 - pdf * (a1*k + a2*k.powi(2) + a3*k.powi(3) + a4*k.powi(4) + a5*k.powi(5));
    
    if x >= 0.0 { phi } else { 1.0 - phi }
}