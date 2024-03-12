use regex::Regex;
use std::cmp::min;
use std::fs::read_to_string;

fn check_index_conditions(idx: usize, val_len: usize) -> bool {
    return idx == 3 || (idx < 3 && idx + val_len >= 3) || (idx > 3 && idx - 1 == 3);
}

fn main() {
    let re = Regex::new(r"[0-9]+").unwrap();
    let char_re = Regex::new(r"[^0-9\.]").unwrap();
    let mut res: Vec<i64> = Vec::new();
    let binding = read_to_string("./input.txt").unwrap();
    let lines = binding.lines().collect::<Vec<&str>>();

    for (idx, line) in lines.iter().enumerate() {
        let mats = line.match_indices(&char_re).collect::<Vec<_>>();

        for (loc, _) in mats {
            let mut local_gears: Vec<i64> = Vec::new();
            // a quick scan of input shows numbers are never large than 3 digits. 
            // in left/right adjacency we can use this to our advantage with slicing
            let start = if loc < 3 { 0 } else { loc - 3 }; 
            let end = min(loc + 3, line.len());
            let up = if idx == 0 { 0 } else { idx - 1 };
            let down = min(idx + 1, lines.len() - 1);

            for (ix, val) in &line[start..end+1].match_indices(&re).collect::<Vec<_>>() {
                if check_index_conditions(*ix, val.len()) {
                    local_gears.push(val.parse::<i64>().unwrap())
                }
            }

            if up != idx {
                for (ix, val) in &lines[up][start..end+1].match_indices(&re).collect::<Vec<_>>() {
                    if check_index_conditions(*ix, val.len()) {
                        local_gears.push(val.parse::<i64>().unwrap())
                    }
                }
            }

            if down != idx {
                for (ix, val) in &lines[down][start..end+1].match_indices(&re).collect::<Vec<_>>()  {
                    if check_index_conditions(*ix, val.len()) {
                        local_gears.push(val.parse::<i64>().unwrap())
                    }
                }
            }

            if local_gears.len() == 2 {
                res.push(&local_gears[0] * &local_gears[1])
            }
        }
    }

    println!("{}", res.iter().sum::<i64>())
}
