use std::vec::Vec;
use std::io;

fn main() {
    let inp_n = inp_to_str();
    let inp_x = inp_to_str();

    let _n: u32 = inp_n.trim()
        .parse()
        .unwrap();
    let mut x = str_to_vec(&inp_x);

    x.sort();
    // calculate Q1, Q2, Q3 -> guaranteed to be integers
    let mut v = vec![0f64, 0f64, 0f64];
    find_quartiles(&x, &mut v, 0, &mut 0);
    println!("{}\n{}\n{}", v[1] as u32, v[0] as u32, v[2] as u32);
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

fn find_quartiles(x: &[i32], res: &mut [f64], depth: u32, idx: &mut u32) {
    if depth > 1 {
        return
    }
    let mid = ( (x.len() as u32) / 2 ) as usize;
    let med = median(x, x.len() as u32);
    res[*idx as usize] = med;
    *idx += 1;
    if x.len() % 2 == 0 {
        find_quartiles(&x[..mid], res, depth + 1, idx);
        find_quartiles(&x[mid..], res, depth + 1, idx);
    } else {
        find_quartiles(&x[..mid], res, depth + 1, idx);
        find_quartiles(&x[mid + 1..], res, depth + 1, idx);
    }
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
