solution!(
    "Day 2: Dive!",
    || {
        let data = common::load("day_02");
        let mut position = 0;
        let mut depth = 0;
        
        for line in data.lines() {
            let command = line
                .split_whitespace()
                .collect::<Vec<&str>>();
            let instruction = command[0];
            let value = command[1].parse::<u32>().unwrap();

            match instruction {
                "forward" => position += value,
                "down" => depth += value,
                "up" => depth -= value,
                _ => panic!("error while parsing command"),
            }
        }

        (position * depth).to_string()
    }
);
