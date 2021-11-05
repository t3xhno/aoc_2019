use std::{fs, error};
use day_02::{Arguments, Codes};

fn main() -> Result<(), Box<dyn error::Error>> {
    let arguments = Arguments::new()?;
    let input = fs::read_to_string(&arguments.file)?;
    println!("{}", run(&arguments.solution, &input)?);
    Ok(())
}

fn solve1(input: &str) -> i32 {
    let mut codes = Codes::new(&input);
    codes.calculate()
}

fn solve2(input: &str) -> i32 {
    let mut codes = Codes::new(&input);
    for i in 0..99 {
        for j in 0..99 {
            codes.list[1] = i; codes.list[2] = j;
            if codes.calculate() == 19690720 {
                return codes.list[1] * 100 + codes.list[2];
            };
            codes = Codes::new(&input);
        };
    };
    0
}

fn run(solution: &str, input: &str) -> Result<i32, Box<dyn error::Error>> {
    match solution == "1" {
        true => Ok(solve1(input)),
        _ => Ok(solve2(input)),
    }
}
