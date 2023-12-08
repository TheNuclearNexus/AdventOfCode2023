use std::{cmp::Ordering, collections::HashMap, error::Error};


#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Debug)]

enum HandKind {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Hand {
    kind: HandKind,
    cards: [i32; 5],
    bid: i32,
}

impl Hand {
    fn cmp(self: &&Hand, b: &&Hand) -> Ordering {
        if self.kind != b.kind {
            return if (self.kind as i32) - (b.kind as i32) > 0 {
                Ordering::Greater
            } else {
                Ordering::Less
            };
        } else {
            for i in 0..5 {
                if self.cards[i] != b.cards[i] {
                    return if self.cards[i] > b.cards[i] {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    };
                }
            }
        }
        return Ordering::Equal;
    }
}

pub fn to_hand(input: &str, special_joker: bool) -> Result<Vec<Hand>, Box<dyn Error>> {
    let hands = input
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");

            let cards: Vec<i32> = parts.next().unwrap().chars().map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => if special_joker {1} else {11},
                'T' => 10,
                _ => c.to_digit(10).unwrap(),
            } as i32).collect();
            let bid = parts.next().unwrap().parse::<i32>().unwrap();

            let mut counts: HashMap<i32, i32> = HashMap::new();

            for c in cards.iter() {
                if counts.contains_key(c) {
                    counts.insert(*c, counts.get(c).unwrap() + 1);
                } else {
                    counts.insert(*c, 1);
                }
            }

            if special_joker && counts.contains_key(&1) && counts.len() > 1 {
                let joker_count = counts.remove(&1).unwrap();

                let mut entries: Vec<(i32, i32)> = counts.clone().into_iter().collect();
                entries.sort_by(|a, b| a.1.cmp(&b.1));

                let (key, value) = entries.last().unwrap();

                counts.insert(*key, value + joker_count);
            }

            let kind = match counts.len() {
                5 => HandKind::HighCard,
                4 => HandKind::OnePair,
                3 => {
                    let mut entries: Vec<(&i32, &i32)> = counts.iter().collect();
                    entries.sort_by(|a, b| a.1.cmp(b.1));
                    let larger = entries.last().unwrap();

                    if larger.1 == &3 {
                        HandKind::ThreeOfAKind
                    } else {
                        HandKind::TwoPair
                    }
                }
                2 => {
                    let entries: Vec<(&i32, &i32)> = counts.iter().collect();
                    let larger = if entries[0].1 > entries[1].1 {
                        entries[0]
                    } else {
                        entries[1]
                    };

                    if larger.1 == &3 {
                        HandKind::FullHouse
                    } else {
                        HandKind::FourOfAKind
                    }
                }
                1 => HandKind::FiveOfAKind,
                _ => panic!(),
            };

            Hand {
                kind,
                cards: cards.as_slice().try_into().unwrap(),
                bid,
            }
        })
        .collect();

    Ok(hands)
}

#[aoc_generator(day7, part1)]
pub fn part1_gen(input: &str) -> Result<Vec<Hand>, Box<dyn Error>> {
    to_hand(
        input,
        false
    )
}

#[aoc_generator(day7, part2)]
pub fn part2_gen(input: &str) -> Result<Vec<Hand>, Box<dyn Error>> {
    to_hand(
        input,
        true
    )
}


fn calculate_game_value(input: &[Hand]) -> i32 {
    let mut input: Vec<&Hand> = input.iter().collect();
    input.sort_by(Hand::cmp);

    let mut total = 0;
    for i in 0..input.len() {
        total += (i as i32 + 1) * input[i].bid;
    }
    total
}

#[aoc(day7, part1)]
pub fn part1(input: &[Hand]) -> i32 {
    calculate_game_value(input)
}

#[aoc(day7, part2)]
pub fn part2(input: &[Hand]) -> i32 {
    calculate_game_value(input)
}

#[cfg(test)]
mod tests {
    use super::{part1_gen, part2_gen, part1, part2};

    #[test]
    pub fn test1() {
        let data = part1_gen(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        )
        .unwrap();

        assert_eq!(part1(&data), 6440);
    }

    #[test]
    pub fn test2() {
        let data = part2_gen(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        )
        .unwrap();

        assert_eq!(part2(&data), 5905);
    }

}
