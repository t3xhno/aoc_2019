use std::{env, fs, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::new()?;
    let input = fs::read_to_string(&config.file)?;
    solve(&config.solution, &input);
    Ok(())
}

struct Config {
    file: String,
    solution: String,
}

impl Config {
    fn new() -> Result<Config, &'static str> {
        let args = env::args().collect::<Vec<String>>();
        match args.len() < 3 {
            true => Err("Needs 2 arguments [Filename + solution number]"),
            _ => Ok(Config {
                file: args[1].clone(),
                solution: args[2].clone(),
            })
        }
    }
}

fn calculate_fuel(current: i32) -> i32 {
    match current <= 0 {
        true => 0,
        _ => current + calculate_fuel(current / 3 - 2)
    }
}

fn solve(flag: &str, input: &str) {
    let total_fuel = input.lines().fold(0, |acc, line| acc + match flag {
        "1" => line.parse::<i32>().unwrap() / 3 - 2,
        _ => calculate_fuel(line.parse::<i32>().unwrap() / 3 - 2)
    });
    println!("{}", total_fuel);
}
