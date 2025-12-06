use std::io;
use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

struct OpPos {
    op: Op,
    pos: usize,
}

struct Col {
    op: Op,
    values: Vec<i64>,
}

fn main() {
    let mut lines: Vec<_> = io::stdin().lock().lines().filter_map(Result::ok).collect();

    let op_line = lines.pop().unwrap();
    let ops: Vec<_> = op_line
        .chars()
        .enumerate()
        .filter_map(|(i, char)| match char {
            '+' => Some(OpPos {
                op: Op::Add,
                pos: i,
            }),
            '*' => Some(OpPos {
                op: Op::Mul,
                pos: i,
            }),
            _ => None,
        })
        .collect();

    let mut cols = Vec::new();
    for i in 0..ops.len() {
        let col_start = ops[i].pos;
        let col_end = if i < ops.len() - 1 {
            ops[i + 1].pos - 1
        } else {
            op_line.len()
        };

        let mut values = Vec::new();

        for j in col_start..col_end {
            let value: String = lines
                .iter()
                .filter_map(|line| line.chars().nth(j))
                .filter(|c| c.is_digit(10))
                .collect();
            values.push(value.parse::<i64>().unwrap());
        }

        cols.push(Col {
            op: ops[i].op,
            values,
        });
    }

    let mut grand_total = 0;

    for col in cols {
        grand_total += match col.op {
            Op::Add => col.values.iter().fold(0i64, |result, value| result + value),
            Op::Mul => col.values.iter().fold(1i64, |result, value| result * value),
        };
    }

    println!("{}", grand_total);
}
