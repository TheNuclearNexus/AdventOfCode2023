use std::cmp::max;
use std::error::Error;
use std::str::Split;
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
struct Game {
    id: i32,
    sets: Vec<Cubes>,
}

#[derive(Debug, Eq, PartialEq)]
struct Cubes {
    red: i32,
    blue: i32,
    green: i32,
}

impl Default for Cubes {
    fn default() -> Self {
        Cubes { red: 0, green: 0, blue: 0 }
    }
}

#[aoc_generator(day2)]
pub fn part1_generator(input: &str) -> Result<Vec<Game>, Box<dyn Error>> {
    let mut games: Vec<Game> = vec![];

    for line in input.lines() {
        let colon = line.find(|c| c == ':').expect("No colon");
        let id = line[5..colon].parse::<i32>().expect("Invalid number");

        let mut sets: Vec<Cubes> = line[colon + 2..line.len()]
            .split("; ")
            .map(
                |set| {
                    let mut cubes = Cubes::default();
                    let totals = set.split(", ");
                    for t in totals {
                        let space = t.find(|c| c == ' ').expect("No space");
                        let num = t[0..space].parse::<i32>().expect("Invalid total");
                        let color = &t[space + 1..t.len()];

                        if color == "red" {
                            cubes.red = num
                        } else if color == "blue" {
                            cubes.blue = num
                        } else if color == "green" {
                            cubes.green = num;
                        }
                    }

                    cubes
                }
            ).collect();

        games.push(Game {
            id,
            sets,
        })
    }

    return Ok(games);
}

#[aoc(day2, part1)]
pub fn part1(input: &[Game]) -> i32 {
    input
        .iter()
        .map(|g| {
            if g.sets.iter().all(|cubes| {
                cubes.red <= 12 && cubes.green <= 13 && cubes.blue <= 14
            }) {
                g.id
            } else {
                0
            }
        })
        .reduce(|a, b| a + b)
        .unwrap()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Game]) -> i32 {
    input
        .iter()
        .map(|g| {
            let mut min_cubes = Cubes::default();

            for c in g.sets.iter() {
                min_cubes.red = max(min_cubes.red, c.red);
                min_cubes.green = max(min_cubes.green, c.green);
                min_cubes.blue = max(min_cubes.blue, c.blue);
            }

            min_cubes.red * min_cubes.green * min_cubes.blue
        })
        .reduce(|a, b| a + b)
        .unwrap()
}


