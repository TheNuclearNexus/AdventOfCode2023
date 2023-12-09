use std::{collections::HashMap, error::Error};

use regex::Regex;
use num::integer::lcm;

pub struct Map {
    instructions: Vec<char>,
    nodes: HashMap<String, (String, String)>,
    starts: Vec<String>
}

#[aoc_generator(day8)]
pub fn day8_generator(input: &str) -> Result<Map, Box<dyn Error>> {
    let mut map = Map {
        instructions: vec![],
        nodes: HashMap::new(),
        starts: vec![]
    };

    let lines: Vec<&str> = input.lines().collect();

    map.instructions = lines[0].chars().collect();

    let node_regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    for i in 2..lines.len() {
        let captures: Vec<&str> = node_regex
            .captures(lines[i])
            .unwrap()
            .iter()
            .filter_map(|mat| {
                if mat.is_some() {
                    Some(mat.unwrap().as_str())
                } else {
                    None
                }
            })
            .collect();
        let key = captures[1].to_owned();

        if key.ends_with("A") {
            map.starts.push(key.clone());
        }

        let connections = (captures[2].to_owned(), captures[3].to_owned());

        map.nodes.insert(key, connections);
    }

    Ok(map)
}

pub fn step_forward(cur: &String, map: &Map, instruction: usize) -> String {
    let direction = &map.instructions[instruction];
    let connections = map.nodes.get(cur).unwrap();

    // println!("cur: {cur}, step: {steps}, next_direction: {direction}");

    if direction == &'L' {
        connections.0.clone()
    } else {
        connections.1.clone()
    }
}

#[aoc(day8, part1)]
pub fn part1(map: &Map) -> i32 {
    let mut steps: i32 = 0;

    let mut cur: String = "AAA".to_owned();
    let mut i: usize = 0;


    while cur != "ZZZ" {
        steps += 1;

        cur = step_forward(&cur, map, i);
        

        i = (i + 1) % map.instructions.len();
    }

    steps
}


#[aoc(day8, part2)]
pub fn part2(map: &Map) -> Option<u64> {

    let start_nodes = map.starts.clone();

    let mut steps = vec![];

    for j in 0..start_nodes.len() {
        let mut cur = start_nodes[j].clone();

        let mut step: u64 = 1;
        let mut i: usize = 0;

        loop {
            cur = step_forward(&cur, map, i);

            if cur.ends_with("Z") {
                steps.push(step);
                break;
            }

            step += 1;
            i = (i + 1) % map.instructions.len();
        }
    }

    let mut sum = 1;
    for i in 0..steps.len() {
        sum = lcm(sum, steps[i]);
    }

    Some(sum)
}


#[cfg(test)]
pub mod tests {
    use crate::day8::{part1, part2};

    use super::day8_generator;


    #[test]
    pub fn test1() {
        let map = day8_generator("RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)").unwrap();

        assert_eq!(part1(&map), 2);
    }

    #[test]
    pub fn test2() {
        let map = day8_generator("LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)").unwrap();

        assert_eq!(part2(&map), Some(6));
    }
}