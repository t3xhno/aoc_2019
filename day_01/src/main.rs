use std::{env, fs, io, process};

fn main() {
    let config = Config::parse_args().unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1);
    });
    let input = get_input(&config.file).unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1);
    });
    if let Err(e) = run(&config.solution, &input) {
        println!("{}", e);
        process::exit(1);
    };
}

struct Config {
    file: String,
    solution: String,
}

impl Config {
    fn parse_args() -> Result<Config, &'static str> {
        let args = env::args().collect::<Vec<String>>();
        match args.len() < 3 {
            true => Err("Needs 2 arguments."),
            _ => Ok(Config {
                file: args[1].clone(),
                solution: args[2].clone(),
            })
        }
    }
}

fn get_input(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn run(arg: &str, input: &str) -> Result<(), &'static str> {
    match arg {
        "1" => solve1(input),
        "2" => solve2(input),
        _ => return Err("Argument is 1 or 2.")
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

fn solve2(input: &str) {
    println!("{}", input);
}