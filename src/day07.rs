use anyhow::Result;
use std::{collections::HashMap, fs, str::FromStr};

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u64,
}

impl FromStr for Hand {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(" ");
        let cards = parts.next().unwrap().chars().collect();
        let bid = parts.next().unwrap().parse()?;
        Ok(Self { cards, bid })
    }
}

fn counter(v: &Vec<char>) -> HashMap<char, usize> {
    let mut res = HashMap::new();
    v.iter().for_each(|c| {
        res.insert(*c, *res.get(c).get_or_insert(&0) + 1);
    });
    res
}

impl Hand {
    fn compare(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        let mut counts1: Vec<usize> = counter(&self.cards).into_values().collect();
        counts1.sort();
        counts1.reverse();

        let mut counts2: Vec<usize> = counter(&other.cards).into_values().collect();
        counts2.sort();
        counts2.reverse();
        for (c1, c2) in counts1.iter().zip(counts2.iter()) {
            if c1 < c2 {
                return Ordering::Less;
            } else if c1 > c2 {
                return Ordering::Greater;
            }
        }

        let card_ord = "23456789TJQKA";

        for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
            let p1 = card_ord.chars().position(|c| c == *c1).unwrap();
            let p2 = card_ord.chars().position(|c| c == *c2).unwrap();
            if p1 < p2 {
                return Ordering::Less;
            } else if p1 > p2 {
                return Ordering::Greater;
            }
        }
        return Ordering::Equal;
    }

    fn compare_joker(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        let mut counter1 = counter(&self.cards);
        let jokers1 = *counter1.remove(&'J').get_or_insert(0);

        let mut counts1: Vec<usize> = counter1.into_values().collect();
        counts1.sort();
        counts1.reverse();
        if jokers1 == 5 {
            counts1.push(5);
        } else if jokers1 > 0 {
            counts1[0] += jokers1;
        }

        let mut counter2 = counter(&other.cards);
        let jokers2 = *counter2.remove(&'J').get_or_insert(0);

        let mut counts2: Vec<usize> = counter2.into_values().collect();
        counts2.sort();
        counts2.reverse();

        if jokers2 == 5 {
            counts2.push(5);
        } else if jokers2 > 0 {
            counts2[0] += jokers2;
        }

        for (c1, c2) in counts1.iter().zip(counts2.iter()) {
            if c1 < c2 {
                return Ordering::Less;
            } else if c1 > c2 {
                return Ordering::Greater;
            }
        }

        let card_ord = "J23456789TQKA";

        for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
            let p1 = card_ord.chars().position(|c| c == *c1).unwrap();
            let p2 = card_ord.chars().position(|c| c == *c2).unwrap();
            if p1 < p2 {
                return Ordering::Less;
            } else if p1 > p2 {
                return Ordering::Greater;
            }
        }
        return Ordering::Equal;
    }
}

fn part1(text: &str) {
    let mut hands: Vec<Hand> = text.lines().map(|l| l.parse::<Hand>().unwrap()).collect();
    hands.sort_by(|h1, h2| h1.compare(h2));
    let res: usize = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * (h.bid as usize))
        .sum();

    println!("{res}");
}

fn part2(text: &str) {
    let mut hands: Vec<Hand> = text.lines().map(|l| l.parse::<Hand>().unwrap()).collect();
    hands.sort_by(|h1, h2| h1.compare_joker(h2));
    let res: usize = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * (h.bid as usize))
        .sum();
    println!("{res}");
}

pub fn compute() {
    let text = fs::read_to_string("inputs/day07.txt").expect("expected readable file");
    part1(&text);
    part2(&text);
}
