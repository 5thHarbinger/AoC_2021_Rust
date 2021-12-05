#[macro_use]
mod common;
mod solutions;

fn main() {
    let sol = [
        solutions::day_01::part_a(),
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
