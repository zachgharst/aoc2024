use std::env;

pub mod day01;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a day number");
        return;
    }

    if args.len() > 3 {
        println!("Too many arguments");
        return;
    }

    let day = &args[1];
    let part = if args.len() == 3 { &args[2] } else { "1" };

    run_day(day, part);
}

fn run_day(day: &str, part: &str) {
    match day {
        "1" => if part == "1" { day01::part1() } else { day01::part2() },
        _ => println!("Day not found"),
    }
}
