use std::cmp::max;
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let mut ranges = input
        .split_once("\n\n")
        .map(|(ranges, _)| {
            ranges
                .lines()
                .map(|range| {
                    range
                        .split_once('-')
                        .map(|(left, right)| {
                            (left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap())
                        })
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .unwrap();

    ranges.sort_by(|(a_left, _), (b_left, _)| a_left.cmp(b_left));

    loop {
        let mut combined = false;

        for i in 0..ranges.len() - 1 {
            let (a_left, a_right) = ranges[i];
            let (b_left, b_right) = ranges[i + 1];
            if a_right >= b_left {
                ranges.remove(i + 1);
                ranges[i] = (a_left, max(a_right, b_right));
                combined = true;
                break;
            }
        }

        if !combined {
            break;
        }
    }

    let fresh_count: i64 = ranges.iter().map(|(left, right)| right - left + 1).sum();

    println!("{}", fresh_count);
}
