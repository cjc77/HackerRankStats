use std::io;
use std::f64::consts::PI;

fn main() {
    let line_1 = inp_to_str();
    let line_2 = inp_to_str();
    let line_3 = inp_to_str();

    let v = str_to_vec(&line_1);
    let mean = v[0];
    let std = v[1];
    let thresh_1 = parse_str(&line_2);
    let thresh_2 = parse_str(&line_3);
    let erf_iterations = 20;

    // println!("{}\n{}\n{}\n{}", mean, std, thresh_1, thresh_2);
    let res_1 = 1f64 - norm_cdf(thresh_1, mean, std, erf_iterations);
    let res_2 = 1f64 - norm_cdf(thresh_2, mean, std, erf_iterations);
    let res_3 = norm_cdf(thresh_2, mean, std, erf_iterations);

    println!("{:.2}\n{:.2}\n{:.2}", res_1 * 100f64, res_2 * 100f64, res_3* 100f64);
}

fn inp_to_str() -> String {
    let mut inp_str = String::new();
    io::stdin().read_line(&mut inp_str).expect("");
    inp_str
}

fn parse_str(inp: &str) -> f64 {
    inp.trim()
        .parse()
        .unwrap()
}

fn str_to_vec(inp_str: &String) -> Vec<f64> {
    let v: Vec<f64> = inp_str.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    v
}

fn norm_cdf(x: f64, mean: f64, std: f64, n: u32) -> f64 {
    let coef = 1f64 / 2f64;
    let erf_res = erf(
        (x - mean) / ( std * 2f64.sqrt() ),
        n
    );
    coef * ( 1f64 + erf_res )
}

fn erf(z: f64, n: u32) -> f64 {
    let mut sums: f64 = 0f64;
    let mut prods: f64 = 1f64; 
    for i in 0..n {
        let i_f64 = i as f64;
        if i != 0 {
            prods *= - (z * z) / i_f64;
        }
        sums += (z * prods) / (2f64 * i_f64 + 1f64)
    }
    sums * ( 2f64 / PI.sqrt())
}
