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

/*Sigma reminds me of the Enigma Nuwell guy on X, must be his twin... lol*/

fn norm_cdf(x: f64) -> f64 {
    let k = 1.0 / (1.0 + 0.2316419 * x.abs());
    let a1 = 0.31938153;
    let a2 = -0.356563782;
    let a3 = 1.781477937;
    let a4 = -1.821255978;
    let a5 = 1.330274429;
    /*I could just make these a tuple. Huh?*/
    let pdf = (-x.powi(2) / 2.0).exp() / (2.0 * PI).sqrt();
    let phi = 1.0 - pdf * (a1*k + a2*k.powi(2) + a3*k.powi(3) + a4*k.powi(4) + a5*k.powi(5));
    
    if x >= 0.0 { phi } else { 1.0 - phi }
}

// Hush! That was some crazy maths

/* If you're following along for learning purposes, here's what each of the methods do from the standard library

· .max() - Returns the larger of two numbers, so 2.max(5) will return 5, 4.max(1) will return 4. There's also a .min() method.

Let me dwell more on this one cause I love it.

Let's assume you don't know about the .max() method, this is how you'd implement it manually:

let diff = s – k;
If diff > 0.0 {
    Diff
} else {
    0.0
}

...but, this is how it actually looks like under the hood in the standard library when you call .max()...

           pub fn max(self, other: f64) -> f64 {
    // NaN cases (Not a Number)
    If self.is_nan() { return self; }
    If other.is_nan() { return other; }
    
    // Regular comparison
    If self > other { self } else { other }
}

Now that looks cool...


· .sqrt() - Calculates the square root of a number
· .exp() - Raises Euler's number (e) to the power of a number, consult your textbooks for that :-)
· .ln() - Computes the natural logarithm (base e) of a number
· .powi() - Raises a number to an integer power, you'll have to pass the integer power through the method like 2.powi(3) for 2 raised to the power of 3
· .abs() - Returns the absolute value (removes negative sign). This is useful if you are interested in magnitudes.
You'll always have to be conscious of BODMAS when working with complex maths like this. To me it's complex!
*/