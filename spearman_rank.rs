use std::io;
use std::collections::HashMap;
use std::cmp::Ordering::Equal;


fn main() {
    let line_1 = inp_to_str();
    let line_2 = inp_to_str();
    let line_3 = inp_to_str();

    let _n = parse_str::<u32>(&line_1);
    let xs = str_to_vec::<f64>(&line_2);
    let ys = str_to_vec::<f64>(&line_3);

    // println!("{}\n{:?}\n{:?}", n, xs, ys);
    let res = spearman_rank(&xs, &ys);
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

fn spearman_rank(xs: &[f64], ys: &[f64]) -> f64 {
    let mut unique_1 = true;
    let mut unique_2 = true;

    let xs_rank = make_rank(&xs, &mut unique_1);
    let ys_rank = make_rank(&ys, &mut unique_2);
    let n = xs.len() as u32;
    coef(&xs_rank, &ys_rank, n, unique_1 == unique_2)
}

fn make_rank(xs: &[f64], unique: &mut bool) -> Vec<i32> {
    let n = xs.len();
    let mut ranks = vec![0; n];
    let mut xs_p = vec![0f64; n];
    let mut xs_map = HashMap::new();

    xs_p[..].clone_from_slice(xs);
    xs_p.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    xs_map.insert(
        String::from(format!("{:.1}", xs_p[0])),
        1
    );
    for i in 1..n {
        let r_c = xs_map.entry(format!("{:.1}", xs_p[i]))
            .or_insert(-1);
        if *r_c != -1 {
            *unique = false;
        }
        *r_c = i as i32 + 1;
    }
    for (i, item) in xs.iter().enumerate() {
        ranks[i] = xs_map[&format!("{:.1}", item)];
    }
    ranks
}

fn coef(rank_x: &[i32], rank_y: &[i32], n: u32, unique: bool) -> f64 {
    if unique {
        let d_is: i32 = rank_x.iter()
            .zip(rank_y)
            .map(|(xi, yi)| (xi - yi).pow(2) )
            .sum();
        let denom = ( n * (n.pow(2) - 1) ) as f64;
        return 1f64 - ( (6 * d_is) as f64 / denom )
    } else {
        return pearson(rank_x, rank_y, n)
    }
}

fn pearson(xs: &[i32], ys: &[i32], n: u32) -> f64 {
    let n = n as f64;
    let x_std = std_dev(&xs);
    let y_std = std_dev(&ys);
    cov(&xs, &ys) / (n * x_std * y_std)
}

fn cov(xs: &[i32], ys: &[i32]) -> f64 {
    let x_mean = mean(&xs);
    let y_mean = mean(&ys);
    let res = xs.iter()
        .zip(ys)
        .map(|(xi, yi)| ((*xi as f64) - x_mean) * ((*yi as f64) - y_mean))
        .sum();
    res
}

fn mean(z: &[i32]) -> f64 {
    let sum: i32 = z.iter().sum();
    let sum = sum as f64;
    sum / (z.len() as f64)
}

fn std_dev(z: &[i32]) -> f64 {
    let n = z.len() as f64;
    let m = mean(z);
    let sse: f64 = z.iter()
        .map(|zi| ( (*zi as f64) - m )
        .powf(2f64)).sum();
    (sse / n).sqrt()
}