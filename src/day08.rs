use anyhow::{Result, anyhow, bail};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::util;

struct Line {
    src: String,
    left: String,
    right: String,
}

impl FromStr for Line {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split_once(" = ").ok_or(anyhow!("Couldn't split {s}"))?;
        let src = parts.0.to_string();
        if let Some((left, right)) = parts.1.replace("(", "").replace(")", "").split_once(", ") {
            Ok(Self {
                src,
                left: left.to_string(),
                right: right.to_string(),
            })
        } else {
            bail!("")
        }
    }
}

fn parse_input(text: &str) -> Result<(Vec<char>, HashMap<String, Line>)> {
    let mut lines = text.lines();
    let left_right: Vec<char> = lines.next().unwrap().chars().collect();
    let mut map = HashMap::new();
    lines.next();
    lines.for_each(|l| {
        let line = l.parse::<Line>().unwrap();
        map.insert(line.src.clone(), line);
    });
    Ok((left_right, map))
}

fn part1(text: &str) {
    if let Ok((left_right, map)) = parse_input(text) {
        let mut pos = "AAA".to_string();
        let mut steps = 0;
        for d in left_right.iter().cycle() {
            let line = map.get(&pos).unwrap();
            match *d {
                'L' => {
                    pos = line.left.clone();
                }
                'R' => {
                    pos = line.right.clone();
                }
                _ => panic!(""),
            }
            steps += 1;
            if pos == "ZZZ" {
                break;
            }
        }
        println!("part 1: {steps}");
    }
}

#[derive(Clone, Debug)]
struct Lasso {
    start: Vec<(String, usize)>,
    cycle: Vec<(String, usize)>,
}

fn lasso_starting_at(start_pos: String, map: &HashMap<String, Line>, left_right: &[char]) -> Lasso {
    let mut seen: HashSet<(String, usize)> = HashSet::new();
    let mut visits: Vec<(String, usize)> = Vec::new();
    let mut pos = start_pos;
    let mut lrp = 0;
    while !seen.contains(&(pos.clone(), lrp)) {
        seen.insert((pos.clone(), lrp));
        visits.push((pos.clone(), lrp));
        let d = left_right[lrp];
        let line = map.get(&pos).unwrap();
        pos = match d {
            'L' => line.left.clone(),
            'R' => line.right.clone(),
            _ => panic!(""),
        };
        lrp = (lrp + 1) % left_right.len();
    }
    let split = visits
        .iter()
        .enumerate()
        .find(|(_, (s, p))| *s == pos && *p == lrp)
        .unwrap()
        .0;
    let start = if split < 1 {
        vec![]
    } else {
        visits.clone()[0..=split.saturating_sub(1)]
            .iter()
            .map(|e| e.to_owned())
            .collect()
    };

    let cycle = visits.clone()[split..visits.len()]
        .iter()
        .map(|e| e.to_owned())
        .collect();
    Lasso { start, cycle }
}

fn lcm(n1: usize, n2: usize) -> usize {
    let mut x;
    let mut y;
    if n1 > n2 {
        x = n1;
        y = n2;
    } else {
        x = n2;
        y = n1;
    }

    let mut rem = x % y;

    while rem != 0 {
        x = y;
        y = rem;
        rem = x % y;
    }

    n1 * n2 / y
}

fn part2(text: &str) {
    if let Ok((left_right, map)) = parse_input(text) {
        let positions: HashSet<String> = map
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|k| k.to_owned())
            .collect();

        let lassos: Vec<Lasso> = positions
            .iter()
            .map(|p| lasso_starting_at(p.to_string(), &map, &left_right))
            .collect();

        let numbers: Vec<(usize, usize, usize)> = lassos
            .iter()
            .map(|l| {
                let sl = l.start.len();
                let cl = l.cycle.len();
                let positions = l
                    .cycle
                    .iter()
                    .enumerate()
                    .filter(|(_, (p, _))| p.ends_with('Z'))
                    .map(|(i, _)| i)
                    .next()
                    .unwrap();
                (sl, cl, positions)
            })
            .collect();
        println!("{numbers:?}");
        let mut pos = numbers[0].0 + numbers[0].2;
        let mut step = numbers[0].1;
        numbers.iter().skip(1).for_each(|next| {
            while (pos - next.0) % next.1 != next.2 {
                pos += step;
            }
            step = lcm(step, next.1);
        });
        println!("part 2: {pos}");
    }
}

pub fn compute() {
    let text = util::read_input_file(8).unwrap();
    part1(&text);
    part2(&text);
}
