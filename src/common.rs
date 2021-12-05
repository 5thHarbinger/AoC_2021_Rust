use std::io;
use std::io::Write;
use std::fs;

pub fn load(file_name: &str) -> String {
    let file = format!("assets/{}.txt", file_name);
    fs::read_to_string(&file).unwrap_or_else(|_| panic!("Error reading file {}", file))
}

pub fn read() -> String {
    let mut buff = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buff).unwrap();
    while buff.ends_with('\n') || buff.ends_with('\r') {
        buff.pop();
    }
    buff
}

pub struct Solution {
    pub name: String,
    pub run: fn() -> String,
}

impl Solution {
    pub fn new(name: &str, run: fn() -> String) -> Solution {
        Solution {
            name: name.to_owned(),
            run,
        }
    }
}

macro_rules! solution {
    ($name:expr, $code:expr) => {
        use crate::common::{self, Solution};

        pub fn part_a() -> Solution {
            Solution::new($name, $code)
        }
    };

    ($name:expr, $code_a:expr, $code_b:expr) => {
        use crate::common::{self, Solution};

        pub fn part_a() -> Solution {
            Solution::new(&format!("{} - part A", $name), $code_a)
        }

        pub fn part_b() -> Solution {
            Solution::new(&format!("{} - part B", $name), $code_b)
        }
    };
}
