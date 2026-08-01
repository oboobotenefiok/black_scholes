use std::f64::consts::PI;
type F = f64;

pub fn call_price(s: F, k: F, t: F, r: F, sigma: F) -> F {
    if t <= 0.0 {
        return (s - k).max(0.0);
    }

    s * norm_cdf(calculate_d1(s, k, t, r, sigma))
        - k * (-r * t).exp() * norm_cdf(calculate_d1(s, k, t, r, sigma) - sigma * t.sqrt())
}

pub fn put_price(s: F, k: F, t: F, r: F, sigma: F) -> F {
    call_price(s, k, t, r, sigma) - s + k * (-r * t).exp()
}

pub fn delta_call(s: F, k: F, t: F, r: F, sigma: F) -> F {
    if t <= 0.0 {
        return if s > k { 1.0 } else { 0.0 };
    }

    norm_cdf(calculate_d1(s, k, t, r, sigma))
}

fn calculate_d1(s: F, k: F, t: F, r: F, sigma: F) -> F {
    ((s / k).ln() + (r + 0.5 * sigma.powi(2)) * t) / (sigma * t.sqrt())
}

fn norm_cdf(x: F) -> F {
    let (k, a1, a2, a3, a4, a5, pdf) = (
        1.0 / (1.0 + 0.2316419 * x.abs()),
        0.31938153,
        -0.356563782,
        1.781477937,
        -1.821255978,
        1.330274429,
        (-x.powi(2) / 2.0).exp() / (2.0 * PI).sqrt(),
    );

    let phi =
        1.0 - pdf * (a1 * k + a2 * k.powi(2) + a3 * k.powi(3) + a4 * k.powi(4) + a5 * k.powi(5));

    if x >= 0.0 { phi } else { 1.0 - phi }
}
