use std::env;

pub mod day01;
pub mod day02;
pub mod day03;

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

    let result = run_day(day, part);

    match result {
        Ok(r) => println!("{}", r),
        Err(e) => println!("Error: {}", e),
    }
}

fn run_day(day: &str, part: &str) -> Result<String, Box<dyn std::error::Error>> {
    match day {
        "1" => if part == "1" { day01::part1() } else { day01::part2() },
        "2" => if part == "1" { day02::part1() } else { day02::part2() },
        "3" => if part == "1" { day03::part1() } else { day03::part2() },
        _ => Err("Invalid day".into()),
    }
}
