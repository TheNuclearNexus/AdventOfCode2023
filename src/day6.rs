
use regex::Regex;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> i32 {
    let numbers_re = Regex::new(r"(\d+)+").unwrap();

    let numbers: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            numbers_re
                .find_iter(line)
                .filter_map(|mat| mat.as_str().parse::<i32>().ok())
                .collect()
        })
        .collect();

    let times = &numbers[0];
    let distances = &numbers[1];

    let mut total = 0;
    for i in 0..times.len() {
        let mut record_count = 0;

        for held_time in 0..times[i] + 1 {
            let speed = held_time;
            let duration = times[i] - held_time;

            let distance = speed * duration;
            if distance > distances[i] {
                record_count += 1;
            }
        }

        total = if total == 0 {
            record_count
        } else {
            total * record_count
        }
    }

    total
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> i32 {
    let numbers_re = Regex::new(r"(\d+)+").unwrap();

    let numbers: Vec<f64> = input
        .lines()
        .map(|line| {
            numbers_re
                .find_iter(line)
                .map(|mat| mat.as_str())
                .fold("".to_owned(), |mut a, b| {a.push_str(b); a})
                .parse::<f64>()
                .unwrap()
        })
        .collect();

    let b = numbers[0];
    let c = -numbers[1];

    let root = f64::sqrt((b * b) + (4_f64 * c));

    let d1 = (-b + root) / -2_f64;
    let d2 = f64::ceil((-b - root) / -2_f64);

    let total = f64::floor(d2 - d1);

    total as i32
}

#[cfg(test)]
mod tests {
    use super::part2;

    #[test]
    pub fn test() {
        assert_eq!(part2("Time:      7  15   30
Distance:  9  40  200"), 71503)
    }
}