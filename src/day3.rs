use std::collections::HashSet;


fn is_symbol(i: usize, j: usize, lines: &Vec<Vec<char>>) -> bool {
    if i >= lines.len() || j >= lines[i].len() {
        return false;
    }

    let char = lines[i][j];

    !char.is_numeric() && char != '.'
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let mut sum = 0;

    for i in 0..lines.len() {
        let mut cur_number = 0;
        let mut valid = false;

        for j in 0..lines[i].len() {
            let cur_char = lines[i][j];
            if cur_char.is_numeric() {
                cur_number = cur_number * 10 + (cur_char.to_digit(10).unwrap());

                if (i > 0 && j > 0 && is_symbol(i - 1, j - 1, &lines))
                    || (i > 0 && is_symbol(i - 1, j, &lines))
                    || (i > 0 && is_symbol(i - 1, j + 1, &lines))
                    || (j > 0 && is_symbol(i, j - 1, &lines))
                    || is_symbol(i, j + 1, &lines)
                    || (j > 0 && is_symbol(i + 1, j - 1, &lines))
                    || is_symbol(i + 1, j, &lines)
                    || (j > 0 && is_symbol(i + 1, j + 1, &lines))
                {
                    valid = true;
                }
            } else {
                sum += if valid { cur_number } else { 0 };
                cur_number = 0;
                valid = false;
            }
        }
        sum += if valid { cur_number } else { 0 };
    }

    sum as i32
}

pub fn get_number(i: usize, j: usize, lines: &Vec<Vec<char>>) -> Option<u32> {
    let mut num = 0;

    if i >= lines.len() || j >= lines[i].len() {
        return None
    }

    println!("{i} {j}");

    if !lines[i][j].is_numeric() {
        return None;
    }

    let mut start = 0;
    let mut end = 0;
    for k in (0..j + 1).rev() {
        if !lines[i][k].is_numeric() {
            start = k + 1;
            break;
        }
    }
    for k in j..lines[i].len() {
        let c: char = lines[i][k];
        if !c.is_numeric() {
            break;
        }
        end = k;
    }

    for k in start..end + 1 {
        num = num * 10 + (lines[i][k].to_digit(10).unwrap());
    }

    if num != 0 {
        println!("{num}");
        Some(num)
    } else {
        None
    }
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let mut sum = 0;

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let cur_char = lines[i][j];
            if cur_char == '*' {

                let mut adjacent: HashSet<u32> = HashSet::new();

                for y in -1..(2 as i32) {
                    for x in -1..(2 as i32) {
                        if (i == 0 && y < 0) || (j == 0 && x < 0) {
                            continue;
                        }
                        // println!("{y} {x}");

                        let num = get_number(((i as i32) + y) as usize, ((j as i32) + x) as usize, &lines);
                        if num.is_some() {
                            adjacent.insert(num.unwrap());
                        }
                    }
                }

                if adjacent.len() > 1 {
                    let total = adjacent.into_iter().reduce(|a, b| a * b).unwrap();
                    println!("({i}, {j}) = {total}");
                    sum += total
                }
            }
        }
    }
    sum as i32
}

