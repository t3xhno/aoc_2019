use std::{fs, error};
use day_02::{Arguments, Codes};

fn main() -> Result<(), Box<dyn error::Error>> {
    let arguments = Arguments::new()?;
    let input = fs::read_to_string(&arguments.file)?;
    println!("{}", solve(&input));
    Ok(())
}

fn solve(input: &str) -> i32 {
    let mut codes = Codes::new(input);
    while codes.opcode != 99 {
        match codes.opcode {
            1 => codes.list[codes.dest] = codes.first + codes.second,
            _ => codes.list[codes.dest] = codes.first * codes.second,
        };
        codes.next();
    };
    codes.list[0]
}
