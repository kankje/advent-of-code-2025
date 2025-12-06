use regex::Regex;
use std::io;
use std::io::BufRead;

fn main() {
    let mut lines: Vec<_> = io::stdin().lock().lines().filter_map(Result::ok).collect();

    let re = Regex::new(r"([0-9+*]+) *").unwrap();

    let rows: Vec<Vec<_>> = lines
        .iter()
        .map(|line| re.captures_iter(line).map(|c| c[1].to_owned()).collect())
        .collect();

    let mut cols = Vec::new();
    for i in 0..rows.first().unwrap().len() {
        let mut col = Vec::new();
        for j in 0..rows.len() {
            col.push(rows[j][i].clone());
        }
        cols.push(col);
    }

    let mut grand_total = 0;

    for mut col in cols {
        let operator = col.pop().unwrap();
        grand_total += match operator.as_str() {
            "+" => col
                .iter()
                .fold(0i64, |result, value| result + value.parse::<i64>().unwrap()),
            "*" => col
                .iter()
                .fold(1i64, |result, value| result * value.parse::<i64>().unwrap()),
            _ => panic!("Invalid operator: {}", operator),
        };
    }

    println!("{}", grand_total);
}
