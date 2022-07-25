solution!(
    "Day 9: Smoke Basin",
    || {
        let data = common::load("day_09");
        let height_map = parse_height_map(data);
        
        find_lows(&height_map)
            .iter()
            .map(|p| height_map[p.y][p.x] + 1)
            .sum::<u32>()
            .to_string()
    }
);

struct Point {
    x: usize,
    y: usize,
}

fn parse_height_map(data: String) -> Vec<Vec<u32>> {
    data
        .lines()
        .map(|x| x
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>()
        )
        .collect::<Vec<Vec<u32>>>()
}

fn find_lows(height_map: &Vec<Vec<u32>>) -> Vec<Point> {
    let mut low_points: Vec<Point> = Vec::new();

    for (y, line) in height_map.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            let current = height_map[y][x];
            if !(
                (y > 0 && current >= height_map[y - 1][x]) ||
                (y < height_map.len() - 1 && current >= height_map[y + 1][x]) ||
                (x > 0 && current >= height_map[y][x - 1]) ||
                (x < height_map[y].len() - 1 && current >= height_map[y][x + 1])
            ) {
                low_points.push(Point{ x, y });
            }
        }
    }

    low_points
}
