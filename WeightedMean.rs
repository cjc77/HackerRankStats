use std::vec::Vec;
use std::io;

fn main() {
    let inp_n = inp_to_str();
    let inp_x = inp_to_str();
    let inp_w = inp_to_str();

    let _n: u32 = inp_n.trim()
        .parse()
        .unwrap();
    let x = str_to_vec(&inp_x);
    let w = str_to_vec(&inp_w);

    let wt_mn = weighted_mean(&x, &w);
    println!("{:.1}", wt_mn);
}

fn inp_to_str() -> String {
    let mut inp_str = String::new();
    io::stdin().read_line(&mut inp_str).expect("");
    inp_str
}

fn str_to_vec(inp_str: &String) -> Vec<i32> {
    let v: Vec<i32> = inp_str.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    v
}

fn weighted_mean(x: &Vec<i32>, w: &Vec<i32>) -> f64{
    let dot_prod: i32 = x.iter()
        .zip(w)
        .map(|(a, b)| *a * *b)
        .sum();
    let denom: i32 = w.iter().sum();
    (dot_prod as f64) / (denom as f64)
}