use std::{collections::VecDeque, error::Error};

use regex::Regex;

const MATCH: &str = r"Card\s*\d+:\s*((?:\d+\s*)+)\|\s*((?:\d+\s*)+)";

pub struct Card {
    winning_numbers: Vec<i32>,
    given_numbers: Vec<i32>
}

#[aoc_generator(day4)]
pub fn day4_gen(input: &str) -> Result<Vec<Card>, Box<dyn Error>> {
    let numbers = Regex::new(MATCH).expect("Invalid regex!");
    let spaces = Regex::new(r"\s+").expect("Invalid regex!");

    Ok(input
        .lines()
        .filter_map(|l| numbers.captures(l))
        .map(|mat| {
            let winning = mat.get(1).expect("No winning").as_str();
            let given = mat.get(2).expect("No given").as_str();

            let winning: Vec<i32> = spaces
                .replace_all(winning, " ")
                .as_ref()
                .split(" ")
                .filter_map(|n| n.parse::<i32>().ok())
                .collect();
            let given: Vec<i32> = spaces
                .replace_all(given, " ")
                .as_ref()
                .split(" ")
                .filter_map(|n| n.parse::<i32>().ok())
                .collect();

            Card {
                winning_numbers: winning,
                given_numbers: given
            }
        }).collect())
}

#[aoc(day4, part1)]
pub fn part1(input: &[Card]) -> i32 {
    input
        .iter()
        .map(
            |c| c.given_numbers.clone()
                .into_iter()
                .filter(|n| c.winning_numbers.contains(n))
                .collect::<Vec<i32>>().len() as u32
            )
        .filter(|n| n > &0)
        .map(|n| 2_i32.pow(n - 1))
        .reduce(|a,b| a+b).unwrap()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Card]) -> i32 {
    let wins: Vec<usize> = input
        .iter()
        .map(
            |c| c.given_numbers.clone()
                .into_iter()
                .filter(|n| c.winning_numbers.contains(n))
                .collect::<Vec<i32>>().len() as usize
            )
        .collect();

    let mut queue: VecDeque<usize> = (0..wins.len()).collect();
    let mut total_cards = 0;
    while queue.len() > 0 {
        let i = queue.pop_front().unwrap();
        total_cards += 1;

        let num_wins = wins[i];

        for j in (1..num_wins + 1).rev() {
            queue.push_front(i + j)
        }
    }
    

    total_cards
}

#[cfg(test)]
mod tests {

    use super::{part2, day4_gen};

    #[test]
    pub fn test() {
        let input = day4_gen("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").ok().unwrap();
        assert_eq!(part2(&input), 30);
    }
}
