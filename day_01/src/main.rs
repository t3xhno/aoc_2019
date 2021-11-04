use std::{env, fs, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::parse_args()?;
    let input = fs::read_to_string(&config.file)?;
    run(&config.solution, &input)?;
    Ok(())
}

struct Config {
    file: String,
    solution: String,
}

impl Config {
    fn parse_args() -> Result<Config, &'static str> {
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

fn solve1(input: &str) {
    println!(
        "{}",
        input.lines()
            .fold(0, |acc, line| acc + line.parse::<i32>().unwrap() / 3 - 2)
    );
}

fn solve2(_input: &str) {
    unimplemented!()
}