use std::vec::Vec;
use std::io;

fn main() {
    let inp_n = inp_to_str();
    let inp_x = inp_to_str();

    let _n: u32 = inp_n.trim()
        .parse()
        .unwrap();
        
    let x = str_to_vec(&inp_x);
    let ans = std_dev(&x);
    println!("{:.1}", ans);
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

fn mean(x: &[i32]) -> f64 {
    let n = x.len();
    let sum: i32 = x.iter().sum();
    (sum as f64) / (n as f64)
}

fn std_dev(x: &[i32]) -> f64 {
    let n = x.len() as f64;
    let m = mean(x);
    let sse: f64 = x.iter()
        .map(|xi| ( (*xi as f64) - m )
        .powf(2f64)).sum();
    (sse / n).sqrt()
}