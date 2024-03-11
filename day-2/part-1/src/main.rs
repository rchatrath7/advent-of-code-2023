use either::*;
use itertools::Itertools;
use std::fs::read_to_string;

const RED: i64 = 12;
const BLUE: i64 = 14;
const GREEN: i64 = 13;

fn check_valid_game(game: &Vec<Vec<(&str, &str)>>) -> bool {
    for set in game.iter() {
        // println!("{:?}", set);
        for color in set {
            // println!("{:?}", color);
            match color.1 {
                "red" => if color.0.parse::<i64>().unwrap() > RED { return false }, 
                "blue" => if color.0.parse::<i64>().unwrap() > BLUE { return false }, 
                "green" => if color.0.parse::<i64>().unwrap() > GREEN { return false }, 
                &_ => return false,
            }
        }
    }

    return true 
}

fn main() {
    println!(
        "{:?}",
        read_to_string("./input.txt")
            .unwrap()
            .lines()
            .map(|l| {
                l.split(":")
                    .enumerate()
                    .map(|(idx, sp)| {
                        if idx == 0 {
                            Left(sp.split(" ").collect::<Vec<&str>>()[1])
                        } else {
                            Right(
                                sp.split(";")
                                    .map(|s| {
                                        s.split(",")
                                            .map(|x| {
                                                x.split(",")
                                                    .map(|y| {
                                                        y.split(" ")
                                                            .filter(|&x| !x.is_empty())
                                                            .collect_tuple::<(&str, &str)>()
                                                            .unwrap()
                                                    })
                                                    .collect::<Vec<(&str, &str)>>()
                                            })
                                            .flatten()
                                            .collect::<Vec<(&str, &str)>>()
                                    })
                                    .collect::<Vec<_>>(),
                            )
                        }
                    })
                    .collect_tuple::<(_, _)>()
            })
            .map(|l| l.unwrap())
            .map(|l| (l.0.unwrap_left(), l.1.unwrap_right()))
            .filter(|l| check_valid_game(&l.1))
            .map(|l| l.0.to_string().parse::<i64>().unwrap())
            .sum::<i64>()
    )
}
