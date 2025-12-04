use std::io;
use std::io::BufRead;

type CharGrid = Vec<Vec<char>>;

fn main() {
    let lines = io::stdin().lock().lines();

    let mut grid: CharGrid = lines.map(|line| line.unwrap().chars().collect()).collect();

    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    let mut total_removed = 0;

    loop {
        let mut to_remove = Vec::new();

        for y in 0..height {
            for x in 0..width {
                if grid[y as usize][x as usize] != '@' {
                    continue;
                }
                let mut adjacent = 0;
                let dirs = [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                    (0, 1),
                    (-1, 1),
                    (-1, 0),
                ];
                for dir in dirs {
                    if x + dir.0 < 0 || x + dir.0 >= width || y + dir.1 < 0 || y + dir.1 >= height {
                        continue;
                    }
                    if grid[(y + dir.1) as usize][(x + dir.0) as usize] == '@' {
                        adjacent += 1;
                    }
                }
                if adjacent < 4 {
                    to_remove.push((x, y));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for &(x, y) in &to_remove {
            grid[y as usize][x as usize] = '.';
        }

        total_removed += to_remove.len();
    }

    println!("{}", total_removed);
}
