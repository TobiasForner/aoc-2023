use anyhow::{Context, Result, anyhow, bail};
use logos::{Lexer, Logos};
use std::{collections::HashMap, str::FromStr};

use crate::util;

fn parse_map(lex: &mut Lexer<Token>) -> Option<(String, String)> {
    let txt = lex.slice();
    let mut parts = txt.split("-to-");
    println!("{parts:?}");
    Some((
        parts.next().unwrap().to_string(),
        parts.next().unwrap().replace(" map:", ""),
    ))
}

#[derive(Debug, PartialEq)]
struct RangeMap {
    pub src_start: i64,
    pub dst_start: i64,
    pub len: i64,
}

impl FromStr for RangeMap {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        println!("{s}");
        let numbers: Result<Vec<i64>> = s
            .split(" ")
            .map(|c| c.parse::<i64>().context(anyhow!("range parse")))
            .collect();
        let numbers = numbers?;
        Ok(Self {
            src_start: numbers[1],
            dst_start: numbers[0],
            len: numbers[2],
        })
    }
}

impl RangeMap {
    pub fn end(&self) -> i64 {
        self.src_start + self.len - 1
    }
}

type RangeMapLookup = HashMap<(String, String), Vec<RangeMap>>;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \n]")]
enum Token {
    #[regex(r"seeds: (\d+)( \d+)*", |lex|lex.slice().replace("seeds: ","").split(" ").map(|n|n.parse().unwrap()).collect::<Vec<i64>>())]
    Seeds(Vec<i64>),
    #[regex(r"[a-z]+-to-[a-z]+ map:", parse_map)]
    Map((String, String)),
    #[regex(r"\d+ \d+ \d+", |lex|lex.slice().parse().ok())]
    Range(RangeMap),
}

fn map_of_maps(text: &str) -> Result<(Vec<i64>, RangeMapLookup)> {
    let lex = Token::lexer(text);
    let mut seeds = vec![];
    let mut range_map: HashMap<(String, String), Vec<RangeMap>> = HashMap::new();
    let mut src = "".to_string();
    let mut dst = "".to_string();
    for e in lex {
        println!("{e:?}");
        match e {
            Ok(Token::Seeds(mut s)) => seeds.append(&mut s),
            Ok(Token::Map((s, d))) => {
                src = s;
                dst = d;
            }
            Ok(Token::Range(r)) => {
                let ranges = range_map.entry((src.clone(), dst.clone())).or_default();
                ranges.push(r);
            }
            Err(_) => bail!("error"),
        }
    }
    Ok((seeds, range_map))
}

fn lookup(item: &i64, v: &Vec<RangeMap>) -> i64 {
    for r in v {
        if r.src_start <= *item && *item < r.src_start + r.len {
            return r.dst_start + item - r.src_start;
        }
    }
    *item
}

fn part1(text: &str) {
    let parts = [
        "seed",
        "soil",
        "fertilizer",
        "water",
        "light",
        "temperature",
        "humidity",
        "location",
    ];
    if let Ok((mut seeds, range_map)) = map_of_maps(text) {
        println!("{seeds:?}; {range_map:?}");
        parts.windows(2).for_each(|w| {
            let map = range_map
                .get(&(w[0].to_string(), w[1].to_string()))
                .unwrap();
            seeds = seeds.iter().map(|e| lookup(e, map)).collect();
        });
        let res = seeds.iter().min().unwrap();
        println!("{res}");
    }
}

#[derive(Clone, Debug)]
struct SeedRange {
    start: i64,
    len: i64,
}

impl SeedRange {
    fn end(&self) -> i64 {
        self.start + self.len - 1
    }

    fn apply_map(&self, r: &RangeMap) -> (Vec<SeedRange>, Vec<SeedRange>) {
        let mut done = vec![];
        let mut left = vec![];
        let done_start = self.start.max(r.src_start);
        let done_end = self.end().min(r.end());
        if done_start < done_end {
            done.push(Self {
                start: r.dst_start + (done_start - r.src_start),
                len: done_end - done_start + 1,
            });
            if self.start < done_start {
                left.push(Self {
                    start: self.start,
                    len: done_start - self.start,
                });
            }
            if done_end < self.end() {
                left.push(Self {
                    start: done_end + 1,
                    len: self.end() - done_end,
                });
            }
        } else {
            left.push(self.clone())
        }

        (done, left)
    }

    fn apply_maps(&self, v: &Vec<RangeMap>) -> Vec<Self> {
        let mut res = vec![];
        let mut todo = vec![self.clone()];
        for m in v {
            let tmp: (Vec<Vec<SeedRange>>, Vec<Vec<SeedRange>>) =
                todo.iter().map(|s| s.apply_map(m)).unzip();
            todo = tmp.1.into_iter().flatten().collect();
            res.append(&mut tmp.0.into_iter().flatten().collect());
        }
        res.append(&mut todo);
        res
    }
}

fn part2(text: &str) {
    let parts = [
        "seed",
        "soil",
        "fertilizer",
        "water",
        "light",
        "temperature",
        "humidity",
        "location",
    ];
    if let Ok((seeds, range_map)) = map_of_maps(text) {
        //todo
        let mut seed_ranges: Vec<SeedRange> = seeds
            .chunks(2)
            .map(|c| SeedRange {
                start: c[0],
                len: c[1],
            })
            .collect();
        println!("{seed_ranges:?}; {range_map:?}");

        parts.windows(2).for_each(|w| {
            let maps = range_map
                .get(&(w[0].to_string(), w[1].to_string()))
                .unwrap();

            seed_ranges = seed_ranges
                .iter()
                .flat_map(|r| r.apply_maps(maps))
                .collect();
        });
        let res = seed_ranges.iter().map(|r| r.start).min().unwrap();
        println!("{res}");
    }
}

pub fn compute() {
    let text = util::read_input_file(5).unwrap();
    part1(&text);
    part2(&text);
}
