use std::fs::read_to_string;

use regex::Regex;
use regex::RegexSet;
use regex::Match; 

use std::collections::HashMap;

fn collect_digit_from_str<'a>(value: &'a str) -> (&str) {
    let re = Regex::new(r"[0-9]").unwrap();
    let str_to_num = HashMap::from([
        ("one", "1"), 
        ("two", "2"), 
        ("three", "3"), 
        ("four", "4"), 
        ("five", "5"), 
        ("six", "6"), 
        ("seven", "7"), 
        ("eight", "8"), 
        ("nine", "9"),
    ]);

    if re.is_match(value) { 
        return value
    } else { 
        return str_to_num.get(value).unwrap() 
    };
}

fn find_digits<'a>(line: &'a str) -> i64 {
    let patterns = [
        "[0-9]", 
        "one",
        "two", 
        "three", 
        "four", 
        "five",
        "six", 
        "seven", 
        "eight", 
        "nine",
    ];
        let set = RegexSet::new(patterns).unwrap();

    let regexes: Vec<_> = set
        .patterns()
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .collect();

    let mut matches: Vec<(usize, &str)> = set
        .matches(line)
        .into_iter()
        .map(|index| &regexes[index])
        .map(|re| re.find_iter(line).collect::<Vec<Match>>())
        .flatten()
        .map(|m: Match| (m.start(), m.as_str()))
        .collect::<Vec<(usize, &str)>>();

    matches.sort_by_key(|x| x.0);

    let num_regex = regexes.first().unwrap();

    return format!("{}{}", 
        collect_digit_from_str(matches[0].1), 
        collect_digit_from_str(matches[matches.len() - 1].1)
    ).parse::<i64>().unwrap();
}

fn main() {
    println!("{}", read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| find_digits(l))
        .sum::<i64>()
    )
}
