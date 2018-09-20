use std::io;
use std::f64::consts::PI;

fn main() {
    let line_1 = inp_to_str();
    let line_2 = inp_to_str();
    let line_3 = inp_to_str();
    let line_4 = inp_to_str();
    let line_5 = inp_to_str();

    let sample_size = parse_str(&line_1);
    let mean = parse_str(&line_2);
    let std = parse_str(&line_3);
    let dist_perc = parse_str(&line_4);
    let z = parse_str(&line_5);

    let std_prime = std / sample_size.sqrt();

    let A = mean - z * std_prime;
    let B = mean + z * std_prime;

    println!("{:.2}\n{:.2}", A, B);
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
