use std::env;

pub struct Arguments {
    pub file: String,
    pub solution: String,
}

impl Arguments {
    pub fn new() -> Result<Arguments, &'static str> {
        let args = env::args().collect::<Vec<String>>();
        match args.len() < 3 {
            true => Err("Needs 2 arguments [Filename + solution number]"),
            _ => Ok(Arguments {
                file: args[1].clone(),
                solution: args[2].clone(),
            })
        }
    }
}
