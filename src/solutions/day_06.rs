solution!(
    "Day 6: Lanternfish",
    || {
        let data = common::load("day_06");
        let mut fishes = data
            .replace("\n", "")
            .split(",")
            .map(|it| it.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        for _ in 0..80 {
            let mut new_fishes = 0usize;
            fishes = fishes
                .iter()
                .map(|fish| {
                    if fish == &0 {
                        new_fishes += 1;
                        6
                    } else {
                        fish - 1
                    }
                })
                .collect();
            for _ in 0..new_fishes {
                fishes.push(8);
            }
        }

        format!("{:?}", fishes.len())
    }
);
