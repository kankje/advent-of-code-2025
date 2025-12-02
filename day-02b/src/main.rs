use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let mut invalid_sum = 0;

    for value in input.split(',') {
        let (from, to) = value.split_once('-').unwrap();
        let from = from.trim().parse::<i64>().unwrap();
        let to = to.trim().parse::<i64>().unwrap();

        for i in from..=to {
            let s = i.to_string();

            for j in 1..=s.len() / 2 {
                let part = &s[..j];
                let invalid = s
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(j)
                    .all(|chunk| chunk.iter().collect::<String>() == part);

                if invalid {
                    invalid_sum += i;
                    break;
                }
            }
        }
    }

    println!("{}", invalid_sum);
}
