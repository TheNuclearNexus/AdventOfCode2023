use std::error::Error;



#[aoc_generator(day9)]
pub fn day9_generator(input: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    Ok(input
        .lines()
        .map(|l| l
            .split(" ")
            .filter_map(|n| n.parse::<i32>().ok())
            .collect())
        .collect()
    )
}

#[aoc(day9, part1)]
pub fn part1(input: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;

    for history in input.clone().iter_mut() {
        let mut offset = 1;
        let mut next = 0;
        loop {
            next += history[history.len() - offset];
            for i in 0..history.len() - offset {
                history[i] = history[i+1] - history[i];
            }

            if (0..history.len() - offset).all(|i| history[i] == 0) {
                total += next;
                break;
            } else {
                offset += 1;
            }
        }
    }

    total
}

#[aoc(day9, part2)]
pub fn part2(input: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;

    for history in input.clone().iter_mut() {
        history.reverse();
        history.push(0);
        history.reverse();

        let mut offset = 2;

        loop {
            for i in (offset..history.len()).rev() {
                history[i] = history[i] - history[i-1];
            }


            if (offset..history.len()).all(|i| history[i] == 0) {
                break;
            } else {
                offset += 1;
            }
        }

        for j in (2..offset + 1).rev() {
            history[j - 1] -= history[j];
        }
        total += history[1];
    }

    total
}



#[cfg(test)]
pub mod tests {
    use crate::day9::{part1, part2};

    use super::day9_generator;

    const INPUT: &str = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";

    #[test]
    pub fn test1() {
        let data = day9_generator(INPUT).unwrap();

        assert_eq!(part1(&data), 114)
    }

    #[test]
    pub fn test2() {
        let data = day9_generator(INPUT).unwrap();

        assert_eq!(part2(&data), 2)
    }
}