use std::{fs, collections::HashMap};

pub fn part1() {
    let contents = fs::read_to_string("./data/day01.txt").expect("Something went wrong reading the file");

    let mut first: Vec<i32> = Vec::new();
    let mut second: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let mut split = line.split_whitespace();
        first.push(split.next().unwrap().parse().unwrap());
        second.push(split.next().unwrap().parse().unwrap());
    }

    first.sort();
    second.sort();

    let mut result = 0;
    for i in 0..first.len() {
        let diff = (first[i] - second[i]).abs();
        result += diff;
    }

    println!("{}", result);
}

pub fn part2() {
    let contents = fs::read_to_string("./data/day01.txt").expect("Something went wrong reading the file");

    let mut first: HashMap<i32, i32> = HashMap::new();
    let mut second: HashMap<i32, i32> = HashMap::new();
    for line in contents.lines() {
        let mut split = line.split_whitespace();
        let f = split.next().unwrap().parse().unwrap();
        let s = split.next().unwrap().parse().unwrap();

        first.entry(f).and_modify(|e| *e += 1).or_insert(1);
        second.entry(s).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut result = 0;
    for (location, frequency) in &first {
        if second.contains_key(location) {
            result += location * frequency * second.get(location).unwrap();
        }
    }

    println!("{}", result);
}
