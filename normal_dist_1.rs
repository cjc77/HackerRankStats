use std::io;
use std::f64::consts::PI;

fn main() {
    let line_1 = inp_to_str();
    let line_2 = inp_to_str();
    let line_3 = inp_to_str();
    let v_1 = str_to_vec(&line_1);
    let v_2 = str_to_vec(&line_3);

    let mean = v_1[0];
    let std = v_1[1];
    let upp_1 = parse_str(&line_2);
    let low_2 = v_2[0];
    let upp_2 = v_2[1];
    // println!("{}\n{}\n{}\n{}\n{}", mean, std, upp_1, low_2, upp_2);
    let erf_iterations = 20;
    let res_1 = norm_cdf(upp_1, mean, std, erf_iterations);
    let res_2 = norm_cdf(upp_2, mean, std, erf_iterations) - norm_cdf(low_2, mean, std, erf_iterations);
    println!("{:.3}\n{:.3}", res_1, res_2);
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
