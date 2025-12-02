use std::io;
use std::io::BufRead;

fn main() {
    let lines = io::stdin().lock().lines();

    let total_max_voltage: i64 = lines
        .filter_map(Result::ok)
        .map(|mut line| {
            for i in (1..=9).rev() {
                if let Some(i) = line[..=line.len() - 12].find(i.to_string().as_str()) {
                    line = line[i..].to_string();
                    break;
                }
            }
            let mut collected = 1;
            while collected < 12 {
                for i in (1..=9).rev() {
                    if let Some(j) =
                        line[collected..=line.len() - (12 - collected)].find(i.to_string().as_str())
                    {
                        line = format!("{}{}", &line[0..collected], &line[j + collected..]);
                        collected += 1;
                        break;
                    }
                }
            }
            line.truncate(collected);
            line.parse::<i64>().unwrap()
        })
        .sum();

    println!("{}", total_max_voltage);
}
