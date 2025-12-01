use std::io;
use std::io::BufRead;

fn main() {
    let lines = io::stdin().lock().lines();

    let values: Vec<i32> = lines
        .filter_map(Result::ok)
        .map(|line| {
            let letter = line.chars().next().unwrap();
            match letter {
                'L' => -line[1..].parse::<i32>().unwrap(),
                'R' => line[1..].parse::<i32>().unwrap(),
                _ => 0,
            }
        })
        .collect();

    let mut pos = 50;
    let mut zero_count = 0;
    for value in values {
        pos += value;
        while pos < 0 {
            pos += 100;
        }
        while pos > 99 {
            pos -= 100;
        }
        if pos == 0 {
            zero_count += 1;
        }
    }

    println!("{}", zero_count);
}
