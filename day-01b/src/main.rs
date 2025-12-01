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
    for mut value in values {
        while value > 0 {
            pos += 1;
            value -= 1;
            if pos > 99 {
                pos = 0;
            }
            if pos == 0 {
                zero_count += 1;
            }
        }
        while value < 0 {
            pos -= 1;
            value += 1;
            if pos < 0 {
                pos = 99;
            }
            if pos == 0 {
                zero_count += 1;
            }
        }
    }

    println!("{}", zero_count);
}
