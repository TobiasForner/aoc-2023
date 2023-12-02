use anyhow::{anyhow, bail, Context, Result};
use std::fs;
use std::str::FromStr;

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
        return self.red <= other.red && self.green <= other.green && self.blue <= other.blue;
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
        return self.results.iter().all(|r| r.leq(colors));
    }

    fn minimum_necessary(&self) -> Colors {
        let min_red = *self.results.iter().map(|c| c.red).max().get_or_insert(0);
        let min_green = *self.results.iter().map(|c| c.green).max().get_or_insert(0);
        let min_blue = *self.results.iter().map(|c| c.blue).max().get_or_insert(0);
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
        let game_parts: Vec<&str> = s.split(": ").collect();
        println!("{game_parts:?}");
        let id = game_parts[0]
            .split_ascii_whitespace()
            .nth(1)
            .ok_or_else(|| anyhow!("Each game needs a number."))?
            .parse()
            .context(anyhow!("Each game needs a number."))?;
        let results = game_parts[1]
            .split("; ")
            .map(|c| c.parse().expect(&format!("Invalid colors: {c}")))
            .collect();
        Ok(Game { id, results })
    }
}

fn part1(games: &Vec<Game>) {
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

fn part2(games: &Vec<Game>) {
    let res: u32 = games.iter().map(|g| g.minimum_necessary().power()).sum();
    println!("{res}")
}

pub fn compute() {
    let text = fs::read_to_string("inputs/day02.txt").expect("expected readable file");
    let games = text
        .lines()
        .map(|l| {
            l.parse::<Game>()
                .expect(&format!("Line {l} is not a valid game!"))
        })
        .collect();
    part1(&games);
    part2(&games);
}
