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
    },
    || {
        let data = common::load("day_01")
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let get_sum_from_index = |index: usize| -> u32 {
            data[index] + data[index + 1] + data[index + 2]
        };

        let mut n = 0;
        for i in 0..data.len() - 3 {
            if get_sum_from_index(i) < get_sum_from_index(i + 1) {
                n += 1;
            }
        }

        n.to_string()
    }
);
