use std::{fs, error};
use day_02::{Arguments, Codes};

fn main() -> Result<(), Box<dyn error::Error>> {
    let arguments = Arguments::new()?;
    let input = fs::read_to_string(&arguments.file)?;
    let mut codes = Codes::new(&input);
    println!("{}", run(&arguments.solution, &mut codes)?);
    Ok(())
}

fn solve1(codes: &mut Codes) -> i32 {
    codes.calculate()
}

fn solve2(codes: &mut Codes) -> i32 {
    for i in 0..99 {
        for j in 0..99 {
            codes.list[1] = i; codes.list[2] = j;
            if codes.calculate() == 19690720 {
                return codes.list[1] * 100 + codes.list[2];
            };
        };
    };
    0
}

fn run(solution: &str, codes: &mut Codes) -> Result<i32, Box<dyn error::Error>> {
    match solution == "1" {
        true => Ok(solve1(codes)),
        _ => Ok(solve2(codes)),
    }
}
