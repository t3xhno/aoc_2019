mod arguments;

use std::{fs, error};
use arguments::Arguments;

fn main() -> Result<(), Box<dyn error::Error>> {
    let arguments = Arguments::new()?;
    let input = fs::read_to_string(&arguments.file)?;
    solve(&arguments.solution, &input);
    Ok(())
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
