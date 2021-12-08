#[macro_use]
mod common;
mod solutions;

fn main() {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

    let sol = [
        solutions::day_01::part_a(),
        solutions::day_01::part_b(),
        solutions::day_02::part_a(),
        solutions::day_02::part_b(),
        solutions::day_03::part_a(),
        solutions::day_03::part_b(),
        solutions::day_04::part_a(),
        solutions::day_04::part_b(),
        solutions::day_05::part_a(),
        solutions::day_05::part_b(),
        solutions::day_06::part_a(),
        solutions::day_06::part_b(),
        solutions::day_07::part_a(),
    ];

    for (i, item) in sol.iter().enumerate() {
        println!("[{:2}] {}", i, item.name);
    }
    print!("\n❯ ");

    let num = match common::read().parse::<usize>() {
        Ok(i) => i,
        Err(_) => panic!("not a number"),
    };

    if num >= sol.len() {
        panic!("invalid id")
    }

    println!("[*] Running: {}", sol[num].name);
    println!("[+] OUT: {}", (sol[num].run)());
}
