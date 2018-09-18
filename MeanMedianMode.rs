use std::io;
use std::vec::Vec;
use std::cmp::Ordering::Equal;

fn main() {
    let mut inp_n = String::new();
    let mut inp_x = String::new();

    io::stdin().read_line(&mut inp_n).expect("");
    let n: u32 = inp_n.trim().parse().unwrap();

    io::stdin().read_line(&mut inp_x).expect("");
    let mut x: Vec<i32> = inp_x.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    x.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));

    println!("{}", mean(&x, n));
    println!("{}", median(&x, n));
    println!("{}", mode(&x, n));
}

fn mean(x: &[i32], n: u32) -> f64 {
    let sum: i32 = x.iter().sum();
    (sum as f64) / (n as f64)
}

fn median(x: &[i32], n: u32) -> f64 {
    let i1 = ( (n - 1) / 2 ) as usize;
    if n % 2 == 0 {
        let i2 = i1 + 1 as usize;
        (x[i1] + x[i2]) as f64 / 2f64
    } else {
        x[i1] as f64
    }
}

fn mode(x: &[i32], n: u32) -> i32 {
    let mut mode: i32 = x[0];
    let mut max_count: u32 = 1;
    let mut count: u32 = 1;

    for (i, cur) in x.iter().enumerate().skip(1) {
        if *cur == x[i - 1] {
            count += 1;
        } else {
            count = 1;
        }

        if count > max_count {
            mode = *cur;
            max_count = count;
        }
    }
    mode
}