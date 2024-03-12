use itertools::Itertools;
use std::fs::read_to_string;
use std::cmp; 

fn calculate_cube_power_set(game: Vec<(&str, &str)>) -> i64 {
    let mut red = 0; 
    let mut blue = 0; 
    let mut green = 0; 

    for g in game.iter() {
        match g.1 {
            "red" => red = cmp::max(red, g.0.parse::<i64>().unwrap()),
            "blue" => blue = cmp::max(blue, g.0.parse::<i64>().unwrap()),
            "green" => green = cmp::max(green, g.0.parse::<i64>().unwrap()),
            &_ => (), 
        }
    }

    return red * blue * green 
}

fn main() {
    println!("{:?}",
        read_to_string("./input.txt")
            .unwrap()
            .lines()
            .map(|l| {
                l.split(":")
                    .collect::<Vec<&str>>()
                    .last()
                    .unwrap()
                    .split(";")
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
                    .flatten()
                    .collect::<Vec<_>>()
            })
            .map(|l| calculate_cube_power_set(l))
            .sum::<i64>()
    )
}
