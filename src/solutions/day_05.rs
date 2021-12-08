use std::collections::HashMap;

solution!(
    "Day 5: Hydrothermal Venture",
    || {
        let segments = Segment::from_file(common::load("day_05"));
        let mut points: HashMap<Point, usize> = HashMap::new();

        for segment in segments.iter() {
            points = segment.calculate_points(points, false);
        }
        
        points
            .into_values()
            .filter(|v| v >= &2)
            .count()
            .to_string()
    },
    || {
        let segments = Segment::from_file(common::load("day_05"));
        let mut points: HashMap<Point, usize> = HashMap::new();

        for segment in segments.iter() {
            points = segment.calculate_points(points, true);
        }

        points
            .into_values()
            .filter(|v| v >= &2)
            .count()
            .to_string()
    }
);

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn from_str(input: &str) -> Self {
        let coords = input
            .split(",")
            .map(|it| it.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Point {
            x: coords[0],
            y: coords[1],
        }
    }
}

struct Segment {
    a: Point,
    b: Point,
}

impl Segment {
    pub fn from_line(input: &str) -> Self {
        let points = input
            .split(" -> ")
            .collect::<Vec<&str>>();
        Segment {
            a: Point::from_str(points[0]),
            b: Point::from_str(points[1]),
        }
    }

    pub fn from_file(input: String) -> Vec<Segment> {
        let mut segments: Vec<Segment> = Vec::new();
        for line in input.lines() {
            segments.push(Segment::from_line(line));
        }
        segments
    }

    pub fn calculate_points(&self, mut points: HashMap<Point, usize>, diag: bool) -> HashMap<Point, usize> {
        let mut increment = |p| {
            if points.contains_key(&p) {
                *points.get_mut(&p).unwrap() += 1;
            } else {
                points.insert(p, 1);
            }
        };
        if self.a.x == self.b.x {
            for y in get_range(self.a.y, self.b.y) {
                increment(Point { x: self.a.x, y });
            }
        } else if self.a.y == self.b.y {
            for x in get_range(self.a.x, self.b.x) {
                increment(Point { x, y: self.a.y });
            }
        } else if diag && get_diff(self.a.x, self.b.x) == get_diff(self.a.y, self.b.y) {
            let xs = get_range(self.a.x, self.b.x);
            let ys = get_range(self.a.y, self.b.y);
            for i in 0..xs.len() {
                increment(Point { x: xs[i], y: ys[i] });
            }
        }
        points
    }
}

fn get_range(a: usize, b: usize) -> Vec<usize> {
    let range = std::cmp::min(a, b)..=std::cmp::max(a, b);
    if a < b {
        range.rev().collect()
    } else {
        range.collect()
    }
}

fn get_diff(a: usize, b: usize) -> usize {
    (a as isize - b as isize).abs() as usize
}
