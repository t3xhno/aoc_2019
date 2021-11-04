use std::{fs, error};
use day_02::{Arguments, Codes};

fn main() -> Result<(), Box<dyn error::Error>> {
    let arguments = Arguments::new()?;
    let input = fs::read_to_string(&arguments.file)?;
    println!("{}", solve2(&input));
    Ok(())
}

fn solve1(input: &str) -> i32 {
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

fn solve2(input: &str) -> i32 {
    for i in 0..99 {
        for j in 0..99 {
            let mut codes = Codes::new(input);
            codes.list[1] = i; codes.list[2] = j;
            while codes.opcode != 99 {
                match codes.opcode {
                    1 => codes.list[codes.dest] = codes.first + codes.second,
                    _ => codes.list[codes.dest] = codes.first * codes.second,
                };
                codes.next();
            };
            if codes.list[0] == 19690720 {
                return codes.list[1] * 100 + codes.list[2];
            };
        };
    };
    0
}
