use anyhow::{Context, Result, anyhow, bail};
use std::str::FromStr;

use crate::util;

struct Colors {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Colors {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for p in s.split(", ") {
            let Some((count, color)) = p.split_once(" ") else {
                bail!("");
            };
            let count: u32 = count.parse()?;
            match color {
                "red" => red += count,
                "green" => green += count,
                "blue" => blue += count,
                _ => bail!("Invalid color: {color}."),
            }
        }
        Ok(Self { red, green, blue })
    }
}

impl Colors {
    fn leq(&self, other: &Colors) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: u32,
    results: Vec<Colors>,
}

impl Game {
    fn possible_with(&self, colors: &Colors) -> bool {
        self.results.iter().all(|r| r.leq(colors))
    }

    fn minimum_necessary(&self) -> Colors {
        let min_red = self.results.iter().map(|c| c.red).max().unwrap_or(0);
        let min_green = self.results.iter().map(|c| c.green).max().unwrap_or(0);
        let min_blue = self.results.iter().map(|c| c.blue).max().unwrap_or(0);
        Colors {
            red: min_red,
            green: min_green,
            blue: min_blue,
        }
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let game_parts: (&str, &str) = s.split_once(": ").context("Each line needs two parts.")?;
        let id = game_parts
            .0
            .split_once(' ')
            .context("Each game needs a number.")?
            .1
            .parse()
            .context(anyhow!("Each game needs a number."))?;
        let results = game_parts
            .1
            .split("; ")
            .map(|c| c.parse().unwrap_or_else(|_| panic!("Invalid colors: {c}")))
            .collect();
        Ok(Game { id, results })
    }
}

fn part1(games: &[Game]) {
    let res: u32 = games
        .iter()
        .filter(|g| {
            g.possible_with(&Colors {
                red: 12,
                green: 13,
                blue: 14,
            })
        })
        .map(|g| g.id)
        .sum();

    println!("{res}")
}

fn part2(games: &[Game]) {
    let res: u32 = games.iter().map(|g| g.minimum_necessary().power()).sum();
    println!("{res}")
}

pub fn compute() -> Result<()> {
    let text = util::read_input_file(2).unwrap();
    let games = text
        .lines()
        .map(|l| l.parse::<Game>())
        .collect::<Result<Vec<Game>>>()?;
    part1(&games);
    part2(&games);
    Ok(())
}
