use std::io;

fn main() {
    let line_1 = inp_to_str();
    let line_2 = inp_to_str();

    let num_den = str_to_vec(&line_1);
    let n: u32 = line_2.trim()
        .parse()
        .unwrap();
    // println!("{}\n{}\n{}", num, den, inspection);
    let p: f64 = num_den[0] / num_den[1];
    // let res = geom_dist(n, p);
    let res = geom_cdf(1, n, p);
    println!("{:.3}", res);
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

fn geom_cdf(low: u32, n: u32, p: f64) -> f64 {
    let mut sum = 0f64;
    for r in low..n + 1 {
        sum += geom_dist(r, p);
    }
    sum
}

fn geom_dist(n: u32, p: f64) -> f64{
    (1f64 - p).powf( (n - 1).into() ) * p
}
