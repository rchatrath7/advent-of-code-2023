use regex::Regex;
use std::cmp::min;
use std::fs::read_to_string;

fn main() {
    let re = Regex::new(r"[0-9]+").unwrap();
    let char_re = Regex::new(r"[^0-9\.]").unwrap();
    let mut res: Vec<i64> = Vec::new();
    let binding = read_to_string("./input.txt").unwrap();
    let lines = binding.lines().collect::<Vec<&str>>();

    for (idx, line) in lines.iter().enumerate() {
        let mats = line.match_indices(&re).collect::<Vec<_>>();

        for (loc, val) in mats {
            let int_val: i64 = val.parse().unwrap();
            let left = if loc == 0 { 0 } else { loc - 1 };
            let right = min(loc + val.len(), line.len() - 1);
            let up = if idx == 0 { 0 } else { idx - 1 };
            let down = min(idx + 1, lines.len() - 1);

            if char_re.is_match(&line[left..left + 1]) {
                res.push(int_val);
                continue;
            }

            if char_re.is_match(&line[right..right + 1]) {
                res.push(int_val);
                continue;
            }

            if char_re.is_match(&lines[up][left..right + 1]) {
                res.push(int_val);
                continue;
            }

            if char_re.is_match(&lines[down][left..right + 1]) {
                res.push(int_val);
                continue;
            }
        }
    }

    println!("{}", res.iter().sum::<i64>())
}
