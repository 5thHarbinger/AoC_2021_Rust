solution!(
    "Day 9: Smoke Basin",
    || {
        let data = common::load("day_09");
        let heightmap = data
            .lines()
            .map(|x| x
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
            )
            .collect::<Vec<Vec<u32>>>();
        let mut lows = 0;

        for (y, line) in heightmap.iter().enumerate() {
            for (x, _) in line.iter().enumerate() {
                let current = heightmap[y][x];
                if !(
                    (y > 0 && current >= heightmap[y - 1][x]) ||
                    (y < heightmap.len() - 1 && current >= heightmap[y + 1][x]) ||
                    (x > 0 && current >= heightmap[y][x - 1]) ||
                    (x < heightmap[y].len() - 1 && current >= heightmap[y][x + 1])
                ) {
                    lows += current + 1;
                }
            }
        }

        lows.to_string()
    }
);
