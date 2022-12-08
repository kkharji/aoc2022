use aoc2022::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The day to run
    #[arg(default_value_t = 1)]
    day: u8,
}

fn main() {
    let day = Args::parse().day;

    let (one, two) = match day {
        1 => (Day01::part1(), Day01::part2()),
        _ => panic!("There's no day {} ", day),
    };

    println!("************************************************************");
    println!("* Advent of Code: 2022");
    println!("*   Solution for...");
    println!("*     Part One: {}", one);
    println!("*     Part Two: {}", two);
    println!("************************************************************");
}
