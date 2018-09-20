use std::io;

fn main() {
    let line_1 = inp_to_str();
    let v = str_to_vec(&line_1);
    let lam_A = v[0];
    let lam_B = v[1];
    // println!("{} {}", lam_A, lam_B);
    let C_A = daily_cost(160, 40, lam_A);
    let C_B = daily_cost(128, 40, lam_B);
    println!("{:.3}\n{:.3}", C_A, C_B);
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

fn daily_cost(base: i32, coeff: i32, lam: f64) -> f64 {
    (base as f64) + ( (coeff as f64) * (lam + lam.powf(2f64)) )
}