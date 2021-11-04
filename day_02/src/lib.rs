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

#[derive(Debug)]
pub struct Codes {
    pub first: i32,
    pub second: i32,
    pub opcode: i32,
    pub code: usize,
    pub dest: usize,
    pub list: Vec<i32>,
}

impl Codes {
    pub fn new(input: &str) -> Codes {
        let mut list = input.split(",").map(|e| e.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        println!("{}", list[0]);
        list[1] = 12; list[2] = 2;
        Codes {
            list,
            code: 0,
            first: input.split(",").map(|e| e.parse::<i32>().unwrap()).collect::<Vec<i32>>()[1],
            second: input.split(",").map(|e| e.parse::<i32>().unwrap()).collect::<Vec<i32>>()[2],
            opcode: input.split(",").map(|e| e.parse::<i32>().unwrap()).collect::<Vec<i32>>()[0],
            dest: input.split(",").map(|e| e.parse::<i32>().unwrap()).collect::<Vec<i32>>()[3] as usize,
        }
    }

    pub fn next(&mut self) {
        self.code += 4;
        self.opcode = self.list[self.code];
        self.dest = self.list[self.code + 3] as usize;
        self.first = self.list[self.list[self.code + 1] as usize];
        self.second = self.list[self.list[self.code + 2] as usize];
    }
}
