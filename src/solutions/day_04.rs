solution!(
    "Day 4: Giant Squid",
    || {
        let data = common::load("day_04");
        let mut blocks = data
            .split("\n\n")
            .collect::<Vec<&str>>();
        let numbers = blocks
            .remove(0)
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let mut grids: Vec<Grid> = Vec::with_capacity(blocks.len());
        for block in blocks.iter() {
            grids.push(Grid::from_string(5, block))
        }

        let won_grids = resolve(grids, numbers);
        let first = won_grids.first().unwrap();

        (first.last_number.unwrap() * first.get_remaining_sum()).to_string()
    },
    || {
        let data = common::load("day_04");
        let mut blocks = data
            .split("\n\n")
            .collect::<Vec<&str>>();
        let numbers = blocks
            .remove(0)
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let mut grids: Vec<Grid> = Vec::with_capacity(blocks.len());
        for block in blocks.iter() {
            grids.push(Grid::from_string(5, block))
        }

        let won_grids = resolve(grids, numbers);
        let last = won_grids.last().unwrap();

        (last.last_number.unwrap() * last.get_remaining_sum()).to_string()
    }
);

fn resolve(mut grids: Vec<Grid>, numbers: Vec<u32>) -> Vec<Grid> {
    let mut won_grids: Vec<Grid> = Vec::with_capacity(grids.len());

    let mut to_remove: Vec<usize>;
    for number in numbers {
        to_remove = Vec::new();
        for (i, grid) in &mut grids.iter_mut().enumerate() {
            if let None = grid.last_number {
                if grid.check_number(number) && grid.is_win() {
                    grid.last_number = Some(number);
                    to_remove.push(i);
                }
            }
        }
        for i in to_remove {
            won_grids.push(grids[i].to_owned());
        }
    }

    won_grids
}


use std::cell::Cell;

#[derive(Clone)]
struct GridCell {
    value: u32,
    is_checked: Cell<bool>,
}

impl GridCell {
    pub fn new(value: u32) -> Self {
        Self {
            value,
            is_checked: Cell::from(false),
        }
    }
}

#[derive(Clone)]
struct Grid {
    size: usize,
    cells: Vec<GridCell>,
    last_number: Option<u32>
}

impl Grid {
    pub fn from_string(size: usize, data: &str) -> Self {
        let cells = data
            .split_whitespace()
            .map(|n| GridCell::new(n.parse::<u32>().unwrap()))
            .collect::<Vec<GridCell>>();
        if cells.len() != size.pow(2) {
            panic!("element count does not match grid size")
        }
        Self {
            size,
            cells: cells,
            last_number: None
        }
    }

    pub fn check_number(&mut self, n: u32) -> bool{
        for cell in &self.cells {
            if cell.value == n {
                cell.is_checked.set(true);
                return true;
            }
        }
        false
    }

    pub fn is_win(&self) -> bool {
        self.scan(|a, b| b + self.size * a) || self.scan(|a, b| a + self.size * b)
    }

    fn scan<C>(&self, c: C) -> bool
    where C: Fn(usize, usize) -> usize {
        let mut is_win: bool;
        for i in 0..self.size {
            is_win = true;
            for j in 0..self.size {
                if ! self.cells[c(i, j)].is_checked.get() {
                    is_win = false;
                    break;
                }
            }
            if is_win {
                return true
            }
        }
        false
    }

    pub fn get_remaining_sum(&self) -> u32 {
        let mut total = 0;
        for cell in &self.cells {
            if ! cell.is_checked.get() {
                total += cell.value;
            }
        }

        total
    }
}
