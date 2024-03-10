use std::fs::read_to_string;

fn main() {
    println!("{}", read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| l.match_indices(|c: char| c.is_numeric()).collect::<Vec<(usize, &str)>>())
        .map(|l| format!("{}{}", l[0].1, l[l.len() - 1].1).parse::<i64>().unwrap())
        .sum::<i64>()
    )
}

