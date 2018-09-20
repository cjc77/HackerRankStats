use std::io;
use std::f64::consts::PI;

fn main() {
    let line_1 = inp_to_str();
    let line_2 = inp_to_str();
    let line_3 = inp_to_str();
    let line_4 = inp_to_str();

    let max_wt = parse_str(&line_1);
    let num_boxes = parse_str(&line_2);
    let mean = parse_str(&line_3);
    let std = parse_str(&line_4);
    // println!("{}\n{}\n{}\n{}", max_wt, num_boxes, mean, std);

    let mean_prime = num_boxes * mean;
    let std_prime = num_boxes.sqrt() * std;
    let erf_iterations = 20;
    // println!("{}\n{}", mean_prime, std_prime);
    let res = norm_cdf(max_wt, mean_prime, std_prime, erf_iterations);
    println!("{:.4}", res);
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
