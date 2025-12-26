use anyhow::{bail, Context, Error, Result};

use std::{
    collections::{HashMap, VecDeque},
    fs,
    str::FromStr,
};

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn value(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let rest = s[1..s.len() - 1].to_string();
        let mut parts = rest.split(',').map(|p| p[2..p.len()].parse());
        let x = parts.next().unwrap()?;
        let m = parts.next().unwrap()?;
        let a = parts.next().unwrap()?;
        let s = parts.next().unwrap()?;
        Ok(Self { x, m, a, s })
    }
}

#[derive(Debug, Clone, Copy)]
enum CompElem {
    X,
    M,
    A,
    S,
}

impl FromStr for CompElem {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        use CompElem::*;
        Ok({
            match s {
                "x" => X,
                "m" => M,
                "a" => A,
                "s" => S,
                _ => bail!("No element for {s}"),
            }
        })
    }
}

#[derive(Debug)]
enum Comparison {
    Greater(u64, CompElem),
    Smaller(u64, CompElem),
    True,
}

impl FromStr for Comparison {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        use Comparison::*;
        Ok({
            if let Some((elem, n)) = s.split_once('<') {
                let elem = elem.parse()?;
                let n = n.parse()?;
                Smaller(n, elem)
            } else if let Some((elem, n)) = s.split_once('>') {
                let elem = elem.parse()?;
                let n = n.parse()?;
                Greater(n, elem)
            } else {
                True
            }
        })
    }
}

impl Comparison {
    fn eval(&self, part: &Part) -> bool {
        use CompElem::*;
        use Comparison::*;
        match self {
            Greater(n, e) => match *e {
                X => part.x > *n,
                M => part.m > *n,
                A => part.a > *n,
                S => part.s > *n,
            },
            Smaller(n, e) => match *e {
                X => part.x < *n,
                M => part.m < *n,
                A => part.a < *n,
                S => part.s < *n,
            },
            True => true,
        }
    }

    fn split_accepting(&self, part_range: &PartRange) -> (Option<PartRange>, Option<PartRange>) {
        use Comparison::*;

        match self {
            Greater(n, e) => part_range.split_at(*n, e, false),
            Smaller(n, e) => part_range.split_at(*n, e, true),
            True => (Some(part_range.clone()), None),
        }
    }
}
#[derive(Debug)]
struct Rule {
    comparison: Comparison,
    destination: String,
}

impl FromStr for Rule {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(if let Some((comp, dest)) = s.split_once(':') {
            let comparison = comp.parse()?;
            let destination = dest.to_string();
            Self {
                comparison,
                destination,
            }
        } else {
            Self {
                comparison: Comparison::True,
                destination: s.to_string(),
            }
        })
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let s = s.replace('}', "");
        let (name, rest) = s.split_once('{').context("context")?;
        let name = name.to_string();
        let rules = rest
            .split(',')
            .map(|r| r.parse::<Rule>())
            .collect::<Result<_>>()?;
        Ok(Self { name, rules })
    }
}

impl Workflow {
    fn apply(&self, part: &Part) -> Result<String> {
        for r in self.rules.iter() {
            if r.comparison.eval(part) {
                return Ok(r.destination.clone());
            }
        }
        bail!("Couldn't apply workflow {self:?} to {part:?}.")
    }

    fn destinations(&self, part_range: &PartRange) -> Vec<(String, PartRange)> {
        let mut current = vec![part_range.clone()];
        let mut res = vec![];
        for rule in self.rules.iter() {
            let mut next = vec![];
            for range in current.iter() {
                let (acc, rej) = rule.comparison.split_accepting(range);
                if let Some(r) = acc {
                    res.push((rule.destination.clone(), r));
                }
                if let Some(r) = rej {
                    next.push(r);
                }
            }
            current = next;
        }
        res
    }
}

fn parse_input(text: &str) -> Result<(Vec<Workflow>, Vec<Part>)> {
    let mut workflows: Vec<Result<Workflow>> = vec![];
    let mut parts: Vec<Result<Part>> = vec![];

    text.lines().for_each(|l| {
        if l.starts_with('{') {
            parts.push(l.parse());
        } else if l.len() > 0 {
            workflows.push(l.parse());
        }
    });

    Ok((
        workflows.into_iter().collect::<Result<Vec<Workflow>>>()?,
        parts.into_iter().collect::<Result<Vec<Part>>>()?,
    ))
}

fn part1(text: &str) -> Result<()> {
    let (workflows, parts) = parse_input(text)?;
    let mut workflow_map = HashMap::new();
    workflows.into_iter().for_each(|w| {
        workflow_map.insert(w.name.clone(), w);
    });
    let mut queue = VecDeque::new();
    parts
        .into_iter()
        .for_each(|p| queue.push_back(("in".to_string(), p)));

    let mut accepted: Vec<Part> = vec![];
    while let Some((wf, part)) = queue.pop_front() {
        if let Some(wf) = workflow_map.get(&wf) {
            let next = wf.apply(&part)?;
            if next == "A" {
                accepted.push(part);
            } else if next != "R" {
                queue.push_back((next, part));
            }
        }
    }

    let res: u64 = accepted.iter().map(|p| p.value()).sum();

    println!("part 1: {res}");
    Ok(())
}

#[derive(Debug, Clone)]
struct PartRange {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl PartRange {
    fn modify(&self, elem: &CompElem, start: u64, end: u64) -> Self {
        use CompElem::*;
        let mut res = self.clone();
        match elem {
            X => res.x = (start, end),
            M => res.m = (start, end),
            A => res.a = (start, end),
            S => res.s = (start, end),
        };
        res
    }

    fn split_at(&self, split: u64, elem: &CompElem, smaller: bool) -> (Option<Self>, Option<Self>) {
        use CompElem::*;
        let (start, end) = match elem {
            X => self.x,
            M => self.m,
            A => self.a,
            S => self.s,
        };
        let (start, end) = split_range(start, end, split, smaller);

        let res1 = {
            if let Some((start, end)) = start {
                Some(self.modify(elem, start, end))
            } else {
                None
            }
        };

        let res2 = {
            if let Some((start, end)) = end {
                Some(self.modify(elem, start, end))
            } else {
                None
            }
        };
        (res1, res2)
    }

    fn size(&self) -> u64 {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }
}

fn split_range(
    start: u64,
    end: u64,
    split: u64,
    smaller: bool,
) -> (Option<(u64, u64)>, Option<(u64, u64)>) {
    if smaller {
        if start < split {
            let first = Some((start, (split - 1).min(end)));
            if split <= end {
                (first, Some((split.max(start), end)))
            } else {
                (first, None)
            }
        } else {
            (None, Some((start, end)))
        }
    } else {
        if split < end {
            let first = Some((start.max(split + 1), end));
            if start <= split {
                (first, Some((start, split)))
            } else {
                (first, None)
            }
        } else {
            (None, Some((start, end)))
        }
    }
}

fn part2(text: &str) -> Result<()> {
    let (workflows, _) = parse_input(text)?;
    let mut workflow_map = HashMap::new();
    workflows.into_iter().for_each(|w| {
        workflow_map.insert(w.name.clone(), w);
    });
    let mut queue = VecDeque::new();
    queue.push_back((
        "in".to_string(),
        PartRange {
            x: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
            m: (1, 4000),
        },
    ));
    let mut accepted: Vec<PartRange> = vec![];
    while let Some((wf, part_range)) = queue.pop_front() {
        if let Some(wf) = workflow_map.get(&wf) {
            let next = wf.destinations(&part_range);
            next.into_iter().for_each(|(dest, pr)| {
                if dest == "A" {
                    accepted.push(pr);
                } else if dest != "R" {
                    queue.push_back((dest, pr));
                }
            })
        }
    }
    let res: u64 = accepted.into_iter().map(|pr| pr.size()).sum();
    println!("part 2: {res}");
    Ok(())
}

pub fn compute() {
    let text = fs::read_to_string("inputs/day19.txt").expect("expected readable file");
    let res1 = part1(&text);
    if res1.is_err() {
        println!("{res1:?}");
    }
    let res2 = part2(&text);
    if res2.is_err() {
        println!("{res2:?}");
    }
}
