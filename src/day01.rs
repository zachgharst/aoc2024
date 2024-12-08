use std::{collections::HashMap, fs};

pub fn part1() -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("./data/day01.txt")?;

    let mut first: Vec<i32> = Vec::new();
    let mut second: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let mut split = line.split_whitespace();
        first.push(
            split
                .next()
                .ok_or("unexpected missing value in first column")?
                .parse()?,
        );
        second.push(
            split
                .next()
                .ok_or("unexpected missing value in second column")?
                .parse()?,
        );
    }

    first.sort();
    second.sort();

    let mut result = 0;
    for i in 0..first.len() {
        let diff = (first[i] - second[i]).abs();
        result += diff;
    }

    Ok(result.to_string())
}

pub fn part2() -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("./data/day01.txt")?;

    let mut first: HashMap<i32, i32> = HashMap::new();
    let mut second: HashMap<i32, i32> = HashMap::new();

    for line in contents.lines() {
        let mut split = line.split_whitespace();
        let f = split
            .next()
            .ok_or("unexpected missing value in first column")?
            .parse()?;
        let s = split
            .next()
            .ok_or("unexpected missing value in second column")?
            .parse()?;

        first.entry(f).and_modify(|e| *e += 1).or_insert(1);
        second.entry(s).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut result = 0;
    for (location, frequency) in &first {
        if second.contains_key(location) {
            result += location * frequency * second.get(location).unwrap();
        }
    }

    Ok(result.to_string())
}
