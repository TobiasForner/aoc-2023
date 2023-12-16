use anyhow::{Context, Result};
use std::{collections::HashMap, fs};

fn parse_input(text: &str, repeat: usize) -> Result<Vec<Line>> {
    let res = text
        .lines()
        .map(|l| {
            let parts = l.split_once(' ').context("")?;
            let mut elems = parts.0.to_string();
            let mut numbers = parts.1.to_string();
            for _ in 1..repeat {
                elems = elems + "?" + parts.0;
                numbers = numbers + "," + parts.1;
            }
            let line: Vec<char> = elems.chars().collect();
            let last_hash = line
                .iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(|(i, _)| i)
                .max();
            let nums = numbers
                .split(',')
                .map(|n| n.parse::<usize>().context("context"))
                .collect::<Result<Vec<usize>>>()?;
            Ok(Line {
                line,
                nums,
                last_hash,
            })
        })
        .collect();
    res
}

#[derive(Debug)]
struct Line {
    line: Vec<char>,
    nums: Vec<usize>,
    last_hash: Option<usize>,
}

impl Line {
    fn can_have_n_hashes(&mut self, n: usize, lpos: usize) -> bool {
        if lpos + n > self.line.len() || (lpos + n < self.line.len() && self.line[lpos + n] == '#')
        {
            return false;
        }
        for i in lpos..lpos + n {
            if self.line[i] == '.' {
                return false;
            }
        }
        true
    }
    fn combinations_from(&mut self, lpos: usize, npos: usize) -> u128 {
        let line = &self.line;

        let mut lpos = lpos;

        while lpos < line.len() && line[lpos] == '.' {
            lpos += 1;
        }
        if lpos >= line.len() && npos >= self.nums.len() {
            return 1;
        } else if npos >= self.nums.len() {
            if line[lpos..line.len()].iter().any(|c| *c == '#') {
                return 0;
            }
            return 1;
        } else if lpos >= line.len() {
            return 0;
        }
        let rem = self.nums[npos];
        if line[lpos] == '#' {
            //need to extend until current num and continue with a dot

            if self.can_have_n_hashes(rem, lpos) {
                self.combinations_from(lpos + rem + 1, npos + 1)
            } else {
                return 0;
            }
        } else {
            // question mark
            // replace by dot
            let count1 = self.combinations_from(lpos + 1, npos);
            // replace by hash
            let count2 = if self.can_have_n_hashes(rem, lpos) {
                self.combinations_from(lpos + rem + 1, npos + 1)
            } else {
                0
            };

            //println!("{lpos}; {npos}; {count1}; {count2}");
            return count1 + count2;
        }
    }

    fn combinations(&mut self) -> u128 {
        let res = self.combinations_from(0, 0);
        //println!("{self:?}; {res}");
        return res;
    }

    fn can_have_n_hashes_lookup(
        &mut self,
        n: usize,
        lpos: usize,
        n_lookup: &mut HashMap<(usize, usize), bool>,
    ) -> bool {
        if let Some(b) = n_lookup.get(&(n, lpos)) {
            return *b;
        } else {
            let b = {
                if lpos + n > self.line.len()
                    || (lpos + n < self.line.len() && self.line[lpos + n] == '#')
                {
                    return false;
                }
                for i in lpos..lpos + n {
                    if self.line[i] == '.' {
                        return false;
                    }
                }
                true
            };
            n_lookup.insert((n, lpos), b);
            b
        }
    }

    fn next_non_dot_lookup(&self, lpos: usize, nnd: &mut HashMap<usize, usize>) -> usize {
        if let Some(p) = nnd.get(&lpos) {
            *p
        } else {
            let mut res = lpos;
            while res < self.line.len() && self.line[res] == '.' {
                res += 1;
            }
            nnd.insert(lpos, res);
            res
        }
    }

    fn has_hash_after(&self, pos: usize) -> bool {
        if let Some(l) = self.last_hash {
            pos <= l
        } else {
            false
        }
    }
    fn combinations_from_lookup(
        &mut self,
        lpos: usize,
        npos: usize,
        n_lookup: &mut HashMap<(usize, usize), bool>,
        c_lookup: &mut HashMap<(usize, usize), u128>,
        nnd: &mut HashMap<usize, usize>,
    ) -> u128 {
        //println!("call {lpos} {npos}");
        if let Some(n) = c_lookup.get(&(lpos, npos)) {
            return *n;
        } else {
            let n;
            let line = &self.line;

            let lpos = self.next_non_dot_lookup(lpos, nnd);

            if lpos >= line.len() && npos >= self.nums.len() {
                n = 1;
            } else if npos >= self.nums.len() {
                if self.has_hash_after(lpos) {
                    n = 0;
                } else {
                    n = 1;
                }
            } else if lpos >= line.len() {
                n = 0;
            } else {
                let rem = self.nums[npos];
                if line[lpos] == '#' {
                    //need to extend until current num and continue with a dot

                    if self.can_have_n_hashes_lookup(rem, lpos, n_lookup) {
                        n = self.combinations_from_lookup(
                            lpos + rem + 1,
                            npos + 1,
                            n_lookup,
                            c_lookup,
                            nnd,
                        );
                    } else {
                        n = 0;
                    }
                } else {
                    // question mark
                    // replace by dot
                    let count1 =
                        self.combinations_from_lookup(lpos + 1, npos, n_lookup, c_lookup, nnd);
                    // replace by hash
                    let count2 = if self.can_have_n_hashes_lookup(rem, lpos, n_lookup) {
                        self.combinations_from_lookup(
                            lpos + rem + 1,
                            npos + 1,
                            n_lookup,
                            c_lookup,
                            nnd,
                        )
                    } else {
                        0
                    };
                    n = count1 + count2;
                }
            };
            //println!("{lpos},{npos}: {n}");
            c_lookup.insert((lpos, npos), n);
            n
        }
    }

    fn combinations_lookup(
        &mut self,
        n_lookup: &mut HashMap<(usize, usize), bool>,
        c_lookup: &mut HashMap<(usize, usize), u128>,
        nnd: &mut HashMap<usize, usize>,
    ) -> u128 {
        let res = self.combinations_from_lookup(0, 0, n_lookup, c_lookup, nnd);
        //println!("{self:?}; {res}");
        return res;
    }
}

fn part1(text: &str) -> Result<()> {
    let mut lines = parse_input(text, 1)?;

    let res: u128 = lines.iter_mut().map(|l| l.combinations()).sum();
    println!("part 1: {res}");
    Ok(())
}

fn part2(text: &str) -> Result<()> {
    let mut lines = parse_input(text, 5)?;

    let res: u128 = lines
        .iter_mut()
        .map(|l| {
            let mut n_lookup = HashMap::new();
            let mut c_lookup = HashMap::new();
            let mut nnd = HashMap::new();
            l.combinations_lookup(&mut n_lookup, &mut c_lookup, &mut nnd)
        })
        .sum();

    println!("part 2: {res}");
    Ok(())
}

pub fn compute() {
    let text = fs::read_to_string("inputs/day12.txt").expect("expected readable file");
    let res1 = part1(&text);
    if res1.is_err() {
        println!("{res1:?}");
    }

    let _ = part2(&text);
}
