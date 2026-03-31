use anyhow::Result;
use itertools::Itertools;
use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::util;
#[derive(Debug)]
struct Pattern {
    pattern: Vec<Vec<char>>,
}

impl FromStr for Pattern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let pattern = s.lines().map(|l| l.chars().collect()).collect();
        Ok(Self { pattern })
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let text = self
            .pattern
            .iter()
            .map(|l| l.iter().collect::<String>())
            .join("\n");
        write!(f, "{text}")
    }
}

impl Pattern {
    fn left_of_vert_mirror(&self) -> usize {
        for x in 1..self.pattern[0].len() {
            if self.is_vert_mirror(x) {
                return x;
            }
        }
        0
    }

    fn above_of_hor_mirror(&self) -> usize {
        for y in 1..self.pattern.len() {
            if self.is_hor_mirror(y) {
                return y;
            }
        }
        0
    }

    fn value(&self) -> usize {
        self.left_of_vert_mirror() + 100 * self.above_of_hor_mirror()
    }

    fn is_hor_mirror(&self, pos: usize) -> bool {
        let pattern = &self.pattern;
        let height = pos.min(pattern.len() - pos);
        (0..height).all(|p| {
            let top = &pattern[pos - p - 1];
            let bottom = &pattern[pos + p];
            top == bottom
        })
    }

    fn is_vert_mirror(&self, pos: usize) -> bool {
        let pattern = &self.pattern;
        let width = pos.min(pattern[0].len() - pos);
        (0..width).all(|p| {
            let left: Vec<char> = (0..pattern.len())
                .map(|i| pattern[i][pos - p - 1])
                .collect();
            let right: Vec<char> = (0..pattern.len()).map(|i| pattern[i][pos + p]).collect();
            left == right
        })
    }

    fn vert_mirrors(&self) -> HashSet<usize> {
        (1..self.pattern[0].len())
            .filter(|x| self.is_vert_mirror(*x))
            .collect()
    }

    fn hor_mirrors(&self) -> HashSet<usize> {
        (1..self.pattern.len())
            .filter(|y| self.is_hor_mirror(*y))
            .collect()
    }

    fn val_after_correction(&self) -> usize {
        let pattern = &self.pattern;
        let positions = (0..pattern.len())
            .flat_map(|y| (0..pattern[y].len()).map(move |x| (x, y)))
            .collect_vec();
        let old_vrl = self.vert_mirrors();
        let old_hrl = self.hor_mirrors();

        for (x, y) in positions {
            let mut pattern = pattern.clone();
            if pattern[y][x] == '#' {
                pattern[y][x] = '.';
            } else {
                pattern[y][x] = '#';
            }
            let np = Self { pattern };
            let hm = np.hor_mirrors();
            let vm = np.vert_mirrors();

            let hd: HashSet<_> = hm.difference(&old_hrl).collect();
            let vd: HashSet<_> = vm.difference(&old_vrl).collect();
            if hd.len() == 1 && vd.is_empty() {
                return 100 * (*hd.into_iter().next().unwrap());
            } else if hd.is_empty() && vd.len() == 1 {
                return *vd.into_iter().next().unwrap();
            }
        }
        panic!("No new pattern for \n{self}");
    }
}
fn parse_input(text: &str) -> Result<Vec<Pattern>> {
    text.split("\n\n").map(|p| p.parse()).collect()
}

fn part1(text: &str) -> Result<()> {
    let input = parse_input(text)?;
    let res: usize = input.iter().map(|v| v.value()).sum();
    println!("part 1: {res}");
    Ok(())
}

fn part2(text: &str) -> Result<()> {
    let input = parse_input(text)?;
    let res: usize = input.iter().map(|v| v.val_after_correction()).sum();
    println!("part 2: {res}");
    Ok(())
}

pub fn compute() {
    let text = util::read_input_file(13).unwrap();
    let _ = part1(&text);
    let _ = part2(&text);
}
