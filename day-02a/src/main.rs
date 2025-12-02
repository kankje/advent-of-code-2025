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
            if s.len() % 2 != 0 {
                continue;
            }

            let (left, right) = s.split_at(s.len() / 2);
            if left == right {
                invalid_sum += i;
            }
        }
    }

    println!("{}", invalid_sum);
}
