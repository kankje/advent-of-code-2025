use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let (ranges, ids) = input
        .split_once("\n\n")
        .map(|(ranges, ids)| {
            (
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
                    .collect::<Vec<_>>(),
                ids.lines().map(|id| id.parse::<i64>().unwrap()),
            )
        })
        .unwrap();

    let fresh_count = ids
        .filter(|id| ranges.iter().any(|(left, right)| id >= left && id <= right))
        .count();

    println!("{}", fresh_count);
}
