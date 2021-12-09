solution!(
    "Day 8: Seven Segment Search",
    || {
        common::load("day_08")
            .lines()
            .flat_map(|x| x
                .split(" | ")
                .collect::<Vec<&str>>()[1]
                .split(" ")
                .collect::<Vec<&str>>()
            )
            .map(|x| [2, 3, 4, 7].contains(&x.len()) as usize)
            .sum::<usize>()
            .to_string()
    }
);
