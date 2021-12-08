solution!(
    "Day 6: Lanternfish",
    || {
        compute(parse_fishes(common::load("day_06")), 80).to_string()
    },
    || {
        compute(parse_fishes(common::load("day_06")), 256).to_string()
    }
);

fn compute(mut cages: [usize; 9], days: usize) -> usize {
    for _ in 0..days {
        let mut new_cages = [0; 9];
        for i in 1..=8 {
            new_cages[i - 1] = cages[i];
        }
        new_cages[6] += cages[0];
        new_cages[8] = cages[0];
        cages = new_cages;
    }
    cages.iter().sum()
}

fn parse_fishes(data: String) -> [usize; 9] {
    let fishes = data
        .replace("\n", "")
        .split(",")
        .map(|it| it.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    let mut cages = [0; 9];
    for fish in fishes {
        cages[fish as usize] += 1;
    }
    cages
}
