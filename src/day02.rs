use std::fs;

pub fn part1() -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("./data/day02.txt")?;

    let mut result = 0;
    for line in contents.lines() {
        if !is_safe_increasing(line) && !is_safe_decreasing(line) {
            continue;
        }

        result += 1;
    }

    Ok(result.to_string())
}

pub fn part2() -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("./data/day02.txt")?;

    let mut result = 0;
    for line in contents.lines() {
        if !is_safe_increasing_with_dampening(line) && !is_safe_decreasing_with_dampening(line) {
            continue;
        }

        result += 1;
    }

    Ok(result.to_string())
}

fn is_safe_increasing(line: &str) -> bool {
    let parts = line.split_whitespace();

    let mut previous = parts.clone().next().unwrap().parse::<i32>().unwrap() - 1;
    for part in parts {
        let num = part.parse::<i32>().unwrap();

        if num <= previous {
            return false;
        }

        if num - previous > 3 {
            return false;
        }

        previous = num;
    }

    true
}

fn is_safe_decreasing(line: &str) -> bool {
    let parts = line.split_whitespace();

    let mut previous = parts.clone().next().unwrap().parse::<i32>().unwrap() + 1;
    for part in parts {
        let num = part.parse::<i32>().unwrap();

        if num >= previous {
            return false;
        }

        if previous - num > 3 {
            return false;
        }

        previous = num;
    }

    true
}

fn is_safe_increasing_with_dampening(line: &str) -> bool {
    if !is_safe_increasing(line) {
        let nums = line.split_whitespace().collect::<Vec<&str>>();

        for i in 0..nums.len() {
            let mut nums = nums.clone();
            nums.remove(i);

            if is_safe_increasing(&nums.join(" ")) {
                return true;
            }
        }

        return false;
    }

    true
}

fn is_safe_decreasing_with_dampening(line: &str) -> bool {
    if !is_safe_decreasing(line) {
        let nums = line.split_whitespace().collect::<Vec<&str>>();

        for i in 0..nums.len() {
            let mut nums = nums.clone();
            nums.remove(i);

            if is_safe_decreasing(&nums.join(" ")) {
                return true;
            }
        }

        return false;
    }

    true
}
