use std::fs;

mod tasks;

use crate::tasks::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    match args.len() {
        1 | 2 => { panic!("Not enough arguments") }
        _ => {
            let day = args[1].as_str();
            let part = args[2].as_str();
            let data = fs::read_to_string(format!("data/{}.txt", day)).unwrap();

            match day {
                "day01" => {
                    match part {
                        "1" => {
                            let result = day01::solve1(data);
                            println!("{}", result);
                        }
                        "2" => {
                            let result = day01::solve2(data);
                            println!("{}", result);
                        }
                        _ => { panic!("What part?") }
                    }
                }
                "day02" => {
                    match part {
                        "1" => {
                            let result = day02::solve1(data);
                            println!("{}", result);
                        }
                        "2" => {
                            let result = day02::solve2(data);
                            println!("{}", result);
                        }
                        _ => { panic!("What part?") }
                    }
                }
                "day03" => {
                    match part {
                        "1" => {
                            let result = day03::solve1(data);
                            println!("{}", result);
                        }
                        "2" => {
                            let result = day03::solve2(data);
                            println!("{}", result);
                        }
                        _ => { panic!("What part?") }
                    }
                }
                _ => { panic!("Invalid argument") }
            }
        }
    }
}
