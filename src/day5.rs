use std::{error::Error, vec};

use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Range {
    start: u32,
    end: u32
}

impl Range {
    pub fn contains(&self, value: u32) -> bool {
        return value >= self.start && value <= self.end;
    }
}

struct RangeMap {
    source: Range,
    destination: Range
}

impl RangeMap {
    pub fn get(&self, value: u32) -> Option<u32> {
        if !self.source.contains(value) {
            return None;
        }

        let delta = value - self.source.start;

        return Some(self.destination.start + delta);
    }
}

pub struct Data {
    maps: Vec<Vec<RangeMap>>,
    seeds: Vec<u32>
}
impl Default for Data {
    fn default() -> Self {
        Data {maps: vec![], seeds: vec![]}
    }
}

const MAP_REGEX: &str = r"((?:.+)-to-(?:.+) map:\n(\d+ \d+ \d+\n?)+)";

#[aoc_generator(day5)] 
pub fn day5_generator(input: &str) -> Result<Data, Box<dyn Error>> {
    let map_regex: Regex = Regex::new(MAP_REGEX).unwrap();
    
    let mut data = Data::default();


    data.seeds = input.lines().next().unwrap()
        .split_at(6).1
        .split(" ")
        .filter_map(|n| n.parse::<u32>().ok())
        .collect();

    data.maps = map_regex
        .find_iter(input)
        .map(|mat| mat
            .as_str()
            .split_once("\n")
            .unwrap().1
            .split("\n").filter_map(|nums| {
                let nums = nums.split(" ")
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect::<Vec<u32>>();

                if nums.len() < 3 {
                    return None;
                }

                let source = nums[1];
                let destination = nums[0    ];
                let range = nums[2];

                Some(RangeMap {
                    source: Range {
                        start: source,
                        end: source + range - 1
                    },
                    destination: Range {
                        start: destination,
                        end: destination + range - 1
                    }
                })
            }).collect())
        .collect();
        

    Ok(data)
}

fn in_range(value: u32, ranges: &Vec<RangeMap>) -> u32 {
    for range in ranges {
        let new = range.get(value);
        // println!("Is {value} between {} and {}?", range.source.start, range.source.end);
        if new.is_some() {
            return new.unwrap();
        }
    }

    value
}

fn seed_to_location(seed: &u32, input: &Data) -> u32 {
    println!("{seed}");

    let soil = in_range(*seed, &input.maps[0]);
    let fertilizer = in_range(soil, &input.maps[1]);
    let water = in_range(fertilizer, &input.maps[2]);
    let light = in_range(water, &input.maps[3]);
    let temperature = in_range(light, &input.maps[4]);
    let humidity = in_range(temperature, &input.maps[5]);
    let location = in_range(humidity, &input.maps[6]);

    location
}

fn partition(mut values: Vec<Range>, ranges: &Vec<RangeMap>) -> Vec<Range> {
    let mut mapped_values = vec![];

    for range in ranges {
        let mut not_mapped_values: Vec<Range> = vec![];
        while values.len() > 0 {
            let value = values.pop().unwrap();
            let left_inside = value.start < range.source.start && value.end > range.source.start;
            let right_inside = value.end > range.source.end && value.start < range.source.end;

            if left_inside && right_inside {
                println!("L&R:  {:10} --> {:10} --> {:10} --> {:10}", value.start, range.source.start, range.source.end, value.end);

                let left_range = Range {
                    start: value.start,
                    end: range.source.start - 1
                };
                not_mapped_values.push(left_range);

                let mid_range = range.destination;
                mapped_values.push(mid_range);

                let right_range = Range {
                    start: range.source.end + 1,
                    end: value.end
                };
                not_mapped_values.push(right_range);

            } else if left_inside {
                println!("L  :  {:10} --> {:10} --> {:10} --> {:10}", value.start, range.source.start, value.end, range.source.end);
                
                let left_range = Range {
                    start: value.start,
                    end: range.source.start - 1
                };
                not_mapped_values.push(left_range);


                let right_range = Range {
                    start: range.destination.start,
                    end: range.destination.start + (value.end - range.source.start)
                };
                mapped_values.push(right_range);
                
            } else if right_inside {
                println!("  R:  {:10} --> {:10} --> {:10} --> {:10}", range.source.start, value.start, range.source.end, value.end);
                let left_range = Range {
                    start: range.destination.start + (value.start - range.source.start),
                    end: range.destination.end
                };
                mapped_values.push(left_range);

                let right_range = Range {
                    start: range.source.end + 1,
                    end: value.end
                };
                not_mapped_values.push(right_range);
            
            } else if value.start >= range.source.start && value.end <= range.source.end {
                println!("   :  {:10} --> {:10} --> {:10} --> {:10}", range.source.start, value.start, value.end, range.source.end);
            
                mapped_values.push(Range {
                    start: range.destination.start + (value.start - range.source.start),
                    end: range.destination.start + (value.end - range.source.start)
                })
            } else {
                // println!("Non: {:10}..{:10} not in {:10}..{:10}", value.start, value.end, range.source.start, range.source.end);
                not_mapped_values.push(value);
            }
        }
        values = not_mapped_values;
    }

    let new_values = [mapped_values, values].concat();
    new_values
}

#[aoc(day5, part1)]
pub fn part1(input: &Data) -> i32 {
    let locations: Vec<u32> = input.seeds.iter().map(|seed| seed_to_location(seed, &input)).collect();
    *locations.iter().reduce(|a,b| a.min(b)).unwrap() as i32
}

#[aoc(day5, part2)]
pub fn part2(input: &Data) -> i32 {
    let mut seeds: Vec<Range> = vec![];

    for i in 0..(input.seeds.len() / 2) {
        let start = input.seeds[i * 2];
        let range = input.seeds[i * 2 + 1];

        seeds.push(Range {
            start,
            end: start + range
        })
    }

    println!("{:?}", seeds);
    for i in 0..input.maps.len() {
        seeds = partition(seeds, &input.maps[i]);
    }
    seeds.sort_by(|a,b| a.start.cmp(&b.start));    

    seeds[0].start as i32
}

#[cfg(test)]
mod tests {
    use super::{day5_generator, part1};

    #[test]
    pub fn test() {
        let data = day5_generator("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4");

        assert_eq!(part1(&data.unwrap()), 35)
    }
}