use regex::Regex;
use std::fs;

pub fn part1() -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("./data/day03.txt")?;

    let re = Regex::new(r"mul\((?<first>[0-9]+),(?<second>[0-9]+)\)").unwrap();

    let mut result = 0;
    for line in contents.lines() {
        let num_pairs = re.captures_iter(line);
        for pair in num_pairs {
            let first = pair.name("first").unwrap().as_str().parse::<i32>().unwrap();
            let second = pair.name("second").unwrap().as_str().parse::<i32>().unwrap();

            result += first * second;
        }
    }

    Ok(result.to_string())
}

pub fn part2() -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("./data/day03.txt")?;
    let mut lines = contents.lines().collect::<Vec<&str>>().join(" ");

    let re = Regex::new(r"mul\((?<first>[0-9]+),(?<second>[0-9]+)\)").unwrap();

    let mut result = 0;
    while lines.chars().count() > 0 {
        let first_dont = lines.find("don't()");
        let right = match first_dont {
            Some(index) => {
                index
            }
            None => {
                lines.chars().count() - 1
            }
        };

        let substr = &lines[0..right];

        let num_pairs = re.captures_iter(substr);
        for pair in num_pairs {
            let first = pair.name("first").unwrap().as_str().parse::<i32>().unwrap();
            let second = pair.name("second").unwrap().as_str().parse::<i32>().unwrap();

            result += first * second;
        }


        lines = (&lines[right + 1..]).to_string();
        let next_do = lines.find("do()");
        match next_do {
            Some(index) => {
                lines = (&lines[index..]).to_string();
            }
            None => {
                break;
            }
        }
    }

    Ok(result.to_string())
}
