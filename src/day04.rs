use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::util;

#[derive(Debug, Clone)]
struct Card {
    //id: u32,
    winning: HashSet<u32>,
    have: HashSet<u32>,
}

impl FromStr for Card {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(": ");
        //flet id = parts.nth(0).unwrap().parse()?;
        let mut numbers = parts.nth(1).unwrap().split(" | ");
        let winning = numbers
            .nth(0)
            .unwrap()
            .split_ascii_whitespace()
            .map(|n| {
                n.trim()
                    .parse()
                    .unwrap_or_else(|_| panic!("could not parse |{n}|"))
            })
            .collect();
        let have = numbers
            .nth(0)
            .unwrap()
            .split_ascii_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect();
        Ok(Self { winning, have })
    }
}

impl Card {
    fn value(&self) -> u32 {
        let count = self.have_winning_count() as u32;
        if count > 0 { 2_u32.pow(count - 1) } else { 0 }
    }

    fn have_winning_count(&self) -> usize {
        self.winning.intersection(&self.have).count()
    }
}

fn parse_cards(text: &str) -> Vec<Card> {
    text.lines().map(|l| l.parse().unwrap()).collect()
}

fn part1(text: &str) {
    let cards = parse_cards(text);
    let res: u32 = cards.iter().map(|c| c.value()).sum();
    println!("{res}");
}

fn part2(text: &str) {
    let mut cards = parse_cards(text);
    let max = cards.len() - 1;
    let mut counts: HashMap<usize, usize> = HashMap::new();
    (0..=max).for_each(|n| {
        counts.insert(n, 1);
    });
    for (i, c) in cards.iter_mut().enumerate() {
        for j in i + 1..=(i + c.have_winning_count()).min(max) {
            counts.insert(j, counts.get(&i).unwrap() + counts.get(&j).unwrap());
        }
    }
    let res: usize = counts.values().sum();
    println!("{res}");
}

pub fn compute() {
    let text = util::read_input_file(4).unwrap();
    part1(&text);
    part2(&text);
}
