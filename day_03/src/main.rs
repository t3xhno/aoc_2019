use std::{error, fs};
use day_03::Arguments;

fn main() -> Result<(), Box<dyn error::Error>> {
    let arguments = Arguments::new()?;
    let input = fs::read_to_string(&arguments.file)?;
    println!("{}", run(&arguments.solution, &input)?);
    Ok(())
}

fn run(solution: &str, input: &str) -> Result<i32, &'static str> {
    match solution {
        "1" => Ok(solve1(input)),
        "2" => Ok(solve2(input)),
        _ => Err("1 or 2"),
    }
}

fn solve1(input: &str) -> i32 {
    println!("{}", input);
    11
}

fn solve2(input: &str) -> i32 {
    println!("{}", input);
    22
}
