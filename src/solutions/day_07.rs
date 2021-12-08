solution!(
    "Day 7: The Treachery of Whales",
    || {
        let crabs = common::load("day_07")
            .replace("\n", "")
            .split(",")
            .map(|it| it.parse().unwrap())
            .collect::<Vec<usize>>();

        compute(crabs, |a| a).to_string()
    },
    || {
        let crabs = common::load("day_07")
            .replace("\n", "")
            .split(",")
            .map(|it| it.parse().unwrap())
            .collect::<Vec<usize>>();
        
        compute(crabs, |a| a * (a + 1) / 2).to_string()
    }
);

fn compute<C>(crabs: Vec<usize>, c: C) -> usize
where C: Fn(usize) -> usize {
    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();

    let mut min_fuel = None;
    for position in *min..=*max {
        let mut fuel = 0;
        for crab in crabs.iter() {
            fuel += c(get_diff(*crab, position));
        }
        if min_fuel.is_none() || fuel < min_fuel.unwrap() {
            min_fuel = Some(fuel);
        }
    }
    min_fuel.unwrap()
}

fn get_diff(a: usize, b: usize) -> usize {
    (a as isize - b as isize).abs() as usize
}
