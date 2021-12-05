solution!(
    "Day 1: Sonar Sweep",
    || {
        let data = common::load("day_01")
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut n = 0;
        for i in 1..data.len() {
            if data[i - 1] < data[i] {
                n += 1;
            }
        }

        n.to_string()
    }
);
