use std::{env, fs, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::new()?;
    let input = fs::read_to_string(&config.file)?;
    run(&config.solution, &input)?;
    // println!("{}", calculate_fuel(100756));
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

fn run(arg: &str, input: &str) -> Result<(), &'static str> {
    match arg {
        "1" => solve1(input),
        "2" => solve2(input),
        _ => return Err("Solution is 1 or 2.")
    };
    Ok(())
}

fn calculate_fuel(current: i32) -> i32 {
    let current = current / 3 - 2;
    match current <= 0 {
        true => 0,
        _ => current + calculate_fuel(current)
    }
}

fn solve1(input: &str) {
    println!(
        "{}",
        input.lines()
            .fold(0, |acc, line| acc + line.parse::<i32>().unwrap() / 3 - 2)
    );
}

fn solve2(input: &str) {
    println!(
        "{}",
        input.lines()
            .fold(0, |acc, curr| acc + calculate_fuel(curr.parse::<i32>().unwrap()))
    )
}