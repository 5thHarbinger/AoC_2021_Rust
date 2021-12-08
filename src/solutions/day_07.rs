solution!(
    "Day 7: The Treachery of Whales",
    || {
        let crabs = common::load("day_07")
            .replace("\n", "")
            .split(",")
            .map(|it| it.parse().unwrap())
            .collect::<Vec<usize>>();
        let min = crabs.iter().min().unwrap();
        let max = crabs.iter().max().unwrap();

        let mut min_fuel = &crabs.len() * (max - min);
        for position in *min..=*max {
            let mut fuel = 0;
            for crab in crabs.iter() {
                fuel += get_diff(*crab, position);
            }
            if fuel < min_fuel {
                min_fuel = fuel;
            }
        }

        min_fuel.to_string()
    }
);

fn get_diff(a: usize, b: usize) -> usize {
    (a as isize - b as isize).abs() as usize
}
