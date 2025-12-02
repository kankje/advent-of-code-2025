use std::io;
use std::io::BufRead;

fn main() {
    let lines = io::stdin().lock().lines();

    let total_max_voltage: i32 = lines
        .filter_map(Result::ok)
        .map(|line| {
            let mut max_voltage = 0;
            for (i, first) in line.chars().enumerate() {
                for second in line[i + 1..].chars() {
                    let combined = format!("{}{}", first, second).parse::<i32>().unwrap();
                    if combined > max_voltage {
                        max_voltage = combined;
                    }
                }
            }

            max_voltage
        })
        .sum();

    println!("{}", total_max_voltage);
}
