use std::vec::Vec;
use std::io;

fn main() {
    let inp_n = inp_to_str();
    let inp_x = inp_to_str();
    let inp_f = inp_to_str();

    let _n: u32 = inp_n.trim()
        .parse()
        .unwrap();
    let x = str_to_vec(&inp_x);
    let f = str_to_vec(&inp_f);

    let freqs: i32 = f.iter().sum();
    let freqs = freqs as usize;
    let mut s = vec![0; freqs];

    gen_s(&mut s, &x, &f);
    let mut v = vec![0f64, 0f64, 0f64];
    find_quartiles(&s, &mut v, 0, &mut 0);
    let res = v[2] - v[1];
    println!("{:.1}", res);
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

fn gen_s(s: &mut [i32], x: &[i32], f: &[i32]) {
    let mut idx = 0;
    for (i, item) in x.iter().enumerate() {
        let f_item = f[i] as usize;
        for _ in 0..f_item {
            s[idx] = *item;
            idx += 1;
        }
    }
    s.sort();
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
