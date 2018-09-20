use std::io;
use std::f64::consts::E;

fn main() {
    let line_1 = inp_to_str();
    let line_2 = inp_to_str();

    let lam: f64 = line_1.trim()
        .parse()
        .unwrap();
    let k: u32 = line_2.trim()
        .parse()
        .unwrap();
    let res = poisson_dist(k, lam);
    println!("{:.3}", res);
}

fn inp_to_str() -> String {
    let mut inp_str = String::new();
    io::stdin().read_line(&mut inp_str).expect("");
    inp_str
}

fn poisson_dist(k: u32, lam: f64) -> f64 {
    ( lam.powf(k.into()) * E.powf( - lam) ) / factorial(k) as f64
}

fn factorial(n: u32) -> u32 {
    if n <= 1 {
        return 1
    }
    n * factorial(n - 1)
}