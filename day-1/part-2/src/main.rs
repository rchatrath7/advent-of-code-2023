use std::fs::read_to_string;

use regex::Regex;

use std::collections::HashMap;

// stolen from stack overvlow to split a string while keeping 
fn split_keep<'a>(text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(|c: char| c.is_numeric()) {
        if last != index {
            result.push(&text[last..index]);
        }
        result.push(matched);
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    result
}

fn main() {
    let re = Regex::new(r"(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)").unwrap();
    let str_to_num = HashMap::from([
        ("one", 1), 
        ("two", 2), 
        ("three", 3), 
        ("four", 4), 
        ("five", 5), 
        ("six", 6), 
        ("seven", 7), 
        ("eight", 8), 
        ("nine", 9),
    ]);
    let mut res = Vec::<i64>::new(); 

    for line in read_to_string("./input.txt").unwrap().lines() {
        //println!("------------------");
        //println!("{}", line);
        let mut first = -1;
        let mut last = -1; 
        let substrs: Vec<&str> = split_keep(line);

        //println!("{:?}", substrs);


        for sub in substrs {
            let to_num = sub.trim().parse::<i64>(); 

            //println!("{}", sub);
            //println!("{:?}", to_num);

            match to_num {
                Ok(ok) => {
                    if first == -1 {
                        first = ok; 
                    }

                    last = ok; 
                }
                Err(_) => {
                    let plaintext_chunk = sub.match_indices(&re).collect::<Vec<(usize, &str)>>();

                    if plaintext_chunk.len() == 0 {
                        continue 
                    } 

                    let mut fnum = 0; 
                    let mut lnum = 0;
                    
                    match str_to_num.get(plaintext_chunk[0].1) {
                        Some(x) => fnum = *x, 
                        None => {
                            println!("found first match that doesn't map to a number: {}", plaintext_chunk[0].1);
                            return 
                        },
                    }

                    match str_to_num.get(plaintext_chunk[plaintext_chunk.len() - 1].1) {
                        Some(x) => lnum = *x, 
                        None => {
                            println!("found last match that doesn't map to a number: {}", plaintext_chunk[plaintext_chunk.len() - 1].1);
                            return 
                        },
                    }

                    if first == -1 {
                        first = fnum; 
                    } 

                    last = lnum; 
                }
            }
        }

        //println!("{}{}", first.to_string(), last.to_string());
        
        res.push(format!("{}{}", first.to_string(), last.to_string()).parse::<i64>().unwrap())
    }

    //println!("{}", res.iter().sum::<i64>())
}
