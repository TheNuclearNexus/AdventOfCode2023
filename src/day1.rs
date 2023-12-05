fn calculate_number(line: &String) -> i32 {
    let first = line.chars()
        .find(|c| c.is_ascii_digit())
        .expect("Failed to find first!");
    let last = line.chars()
        .rfind(|c| c.is_ascii_digit())
        .expect("Failed to find last!");

    let num = format!("{}{}", first, last)
        .parse::<i32>()
        .expect("Failed to parse int!");

    num
}

fn calculate_calibrations(lines: Vec<String>) -> i32 {
    let mut sum = 0;

    for line in lines {
        let num = calculate_number(&line);
        sum += num;
    }

    sum
}

#[aoc(day1, part1, Chars)]
pub fn part1(input: &str) -> i32 {
    let lines = input.split("\n");

    calculate_calibrations(lines.into_iter().map(|l| l.to_string()).collect())
}

const WORD_TO_DIGIT: &[(&str, char)] = &[
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9')
];

fn check_for_word(line: String, idx: usize) -> Option<char> {
    for (word, digit) in WORD_TO_DIGIT.to_owned() {
        if idx + word.len() > line.len() {
            continue;
        }

        let sub_str = &line[idx..idx + word.len()];
        if sub_str == word {
            return Some(digit);
        }
    }

    None
}

fn get_digit(line: &str, i: usize) -> Option<char> {
    let cur_char = line.chars().nth(i).unwrap();
    if cur_char.is_ascii_digit() {
        return Some(cur_char);
    }

    let digit = check_for_word(line.to_string(), i);
    if digit.is_some() {
        return Some(digit.unwrap());
    }

    None
}

#[aoc(day1, part2, Chars)]
pub fn part2(input: &str) -> i32 {
    let lines = input.split("\n");

    let mut sum = 0;

    for line in lines {
        let mut first = ' ';
        let mut last = ' ';

        for i in 0..line.len() {
            let digit = get_digit(line, i);
            if digit.is_some() {
                first = digit.unwrap();
                break;
            }
        }

        for i in (0..line.len()).rev() {
            let digit = get_digit(line, i);
            if digit.is_some() {
                last = digit.unwrap();
                break;
            }
        }

        let num = format!("{first}{last}")
            .parse::<i32>()
            .expect("Failed to parse!");

        sum += num;
    }

    sum
}


#[cfg(test)]
mod tests {
    use super::part2;

    #[test]
    fn sample1() {
        assert_eq!(part2("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"), 281);
    }
}
