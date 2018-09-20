use std::io;

fn main() {
    let line_1 = inp_to_str();
    let line_2 = inp_to_str();
    let line_3 = inp_to_str();

    let n = parse_str::<u32>(&line_1);
    let xs = str_to_vec::<f64>(&line_2);
    let ys = str_to_vec::<f64>(&line_3);

    // println!("{}\n{:?}\n{:?}", n, xs, ys);
    let res = pearson(&xs, &ys, n);
    println!("{:.3}", res);
}

fn inp_to_str() -> String {
    let mut inp_str = String::new();
    io::stdin().read_line(&mut inp_str).expect("");
    inp_str
}

fn parse_str<T>(inp: &str) -> T
    where T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug
{
    inp.trim()
        .parse()
        .unwrap()
}

fn str_to_vec<T>(inp_str: &String) -> Vec<T>
    where T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug
{
    let v: Vec<T> = inp_str.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    v
}

fn pearson(xs: &[f64], ys: &[f64], n: u32) -> f64 {
    let n = n as f64;
    let x_std = std_dev(&xs);
    let y_std = std_dev(&ys);
    cov(&xs, &ys) / (n * x_std * y_std)
}

fn cov(xs: &[f64], ys: &[f64]) -> f64 {
    let x_mean = mean(&xs);
    let y_mean = mean(&ys);
    let res: f64 = xs.iter()
        .zip(ys)
        .map(|(xi, yi)| (xi - x_mean) * (yi - y_mean))
        .sum();
    res
}

fn mean(z: &[f64]) -> f64 {
    let sum: f64 = z.iter().sum();
    sum / (z.len() as f64)
}

fn std_dev(z: &[f64]) -> f64 {
    let n = z.len() as f64;
    let m = mean(z);
    let sse: f64 = z.iter()
        .map(|zi| ( (*zi as f64) - m )
        .powf(2f64)).sum();
    (sse / n).sqrt()
}
