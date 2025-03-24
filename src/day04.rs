use std::fs;

pub fn part1() -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("./data/day04.txt")?;
    let matrix = contents.lines().collect::<Vec<&str>>();
    let height = matrix.len();
    let width = matrix[0].chars().count();

    let mut result = 0;
    for y in 0..height {
        for x in 0..width {
            let c = get_char_at_position(matrix[y], x);
            if c == 'X' {
                result += count_matches(matrix.clone(), height, width, x, y);
            }
        }
    }

    Ok(result.to_string())
}

pub fn part2() -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("./data/day04.txt")?
        .lines()
        .collect::<Vec<&str>>()
        .join(" ");

    let result = "";
    Ok(result.to_string())
}

fn get_char_at_position(s: &str, pos: usize) -> char {
    s.chars().nth(pos).unwrap()
}

fn count_matches(matrix: Vec<&str>, height: usize, width: usize, x: usize, y: usize) -> i32 {
    let mut matches = 0;

    // look left for string "MAS"
    if x >= 3 {
        let mut c = get_char_at_position(matrix[y], x - 1);

        if c == 'M' {
            c = get_char_at_position(matrix[y], x - 2);
            if c == 'A' {
                c = get_char_at_position(matrix[y], x - 3);
                if c == 'S' {
                    matches += 1;
                }
            }
        }
    }

    // look right for string "MAS"
    if x + 3 < width {
        let mut c = get_char_at_position(matrix[y], x + 1);

        if c == 'M' {
            c = get_char_at_position(matrix[y], x + 2);
            if c == 'A' {
                c = get_char_at_position(matrix[y], x + 3);
                if c == 'S' {
                    matches += 1;
                }
            }
        }
    }

    // look up for string "MAS"
    if y >= 3 {
        let mut c = get_char_at_position(matrix[y - 1], x);

        if c == 'M' {
            c = get_char_at_position(matrix[y - 2], x);
            if c == 'A' {
                c = get_char_at_position(matrix[y - 3], x);
                if c == 'S' {
                    matches += 1;
                }
            }
        }
    }

    // look down for string "MAS"
    if y + 3 < height {
        let mut c = get_char_at_position(matrix[y + 1], x);

        if c == 'M' {
            c = get_char_at_position(matrix[y + 2], x);
            if c == 'A' {
                c = get_char_at_position(matrix[y + 3], x);
                if c == 'S' {
                    matches += 1;
                }
            }
        }
    }

    // look up-left for string "MAS"
    if x >= 3 && y >= 3 {
        let mut c = get_char_at_position(matrix[y - 1], x - 1);

        if c == 'M' {
            c = get_char_at_position(matrix[y - 2], x - 2);
            if c == 'A' {
                c = get_char_at_position(matrix[y - 3], x - 3);
                if c == 'S' {
                    matches += 1;
                }
            }
        }
    }

    // look up-right for string "MAS"
    if x + 3 < width && y >= 3 {
        let mut c = get_char_at_position(matrix[y - 1], x + 1);

        if c == 'M' {
            c = get_char_at_position(matrix[y - 2], x + 2);
            if c == 'A' {
                c = get_char_at_position(matrix[y - 3], x + 3);
                if c == 'S' {
                    matches += 1;
                }
            }
        }
    }

    // look down-left for string "MAS"
    if x >= 3 && y + 3 < height {
        let mut c = get_char_at_position(matrix[y + 1], x - 1);

        if c == 'M' {
            c = get_char_at_position(matrix[y + 2], x - 2);
            if c == 'A' {
                c = get_char_at_position(matrix[y + 3], x - 3);
                if c == 'S' {
                    matches += 1;
                }
            }
        }
    }

    // look down-right for string "MAS"
    if x + 3 < width && y + 3 < height {
        let mut c = get_char_at_position(matrix[y + 1], x + 1);

        if c == 'M' {
            c = get_char_at_position(matrix[y + 2], x + 2);
            if c == 'A' {
                c = get_char_at_position(matrix[y + 3], x + 3);
                if c == 'S' {
                    matches += 1;
                }
            }
        }
    }

    matches
}
