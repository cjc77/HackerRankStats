use std::io;

fn main() {
    let inp = inp_to_str();
    let inp_vec = str_to_vec(&inp);

    let p = inp_vec[0] / 100f64;
    let n = inp_vec[1];

    let max_2 = binom_cdf(0, 2, n as u32, p);
    let min_2 = binom_cdf(2, n as u32, n as u32, p);
    println!("{:.3}\n{:.3}", max_2, min_2);
}

fn inp_to_str() -> String {
    let mut inp_str = String::new();
    io::stdin().read_line(&mut inp_str).expect("");
    inp_str
}

fn str_to_vec(inp_str: &String) -> Vec<f64> {
    let v: Vec<f64> = inp_str.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    v
}

fn binom_cdf(low: u32, high: u32, n: u32, p: f64) -> f64 {
    let mut sum: f64 = 0f64;
    for r in low..high + 1 {
        sum += binom(r, n, p);
    }
    sum
}

fn binom(x: u32, n: u32, p: f64) -> f64 {
    let coeff: f64 = n_choose_k(n, x).into();
    coeff * p.powf(x.into()) * (1f64 - p).powf( (n - x).into() )
}

fn n_choose_k(n: u32, k: u32) -> u32 {
    fact(n) / ( fact(k) * fact(n - k) )
}

fn fact(n: u32) -> u32 {
    if n <= 1 {
        return 1
    }
    n * fact(n - 1)
}
