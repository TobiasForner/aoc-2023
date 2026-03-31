use anyhow::Result;
use itertools::Itertools;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    ops::IndexMut,
};

use crate::util::{self, lcm};

#[derive(Debug, Clone)]
enum Module {
    Broadcaster(Vec<usize>), //output
    Button(Vec<usize>),
    FlipFlop(Vec<usize>, bool),                    //output,state
    Conjunction(Vec<usize>, HashMap<usize, bool>), //output,memory,input_count
    Stub,
}

impl Module {
    fn add_output(&mut self, output: usize) {
        use Module::*;
        match self {
            Broadcaster(out) => out.push(output),
            Button(out) => out.push(output),
            FlipFlop(out, _) => out.push(output),
            Conjunction(out, _) => out.push(output),
            Stub => {}
        }
    }

    fn state_description(&self) -> String {
        use Module::*;
        let empty = "".to_string();
        match self {
            Broadcaster(_) => empty,
            Button(_) => empty,
            FlipFlop(_, state) => {
                if *state {
                    "t".to_string()
                } else {
                    "f".to_string()
                }
            }
            Conjunction(_, memory) => memory
                .iter()
                .sorted()
                .map(|(_, b)| if *b { "t" } else { "f" })
                .join(","),
            Stub => empty,
        }
    }

    fn add_input(&mut self, input: usize) {
        use Module::*;
        match self {
            Broadcaster(_) => {}
            Button(_) => {}
            FlipFlop(_, _) => {}
            Conjunction(_, memory) => {
                memory.insert(input, false);
            }
            Stub => {}
        }
    }

    fn outputs(&self) -> Vec<usize> {
        use Module::*;
        match self {
            Broadcaster(out) => out.clone(),
            Button(out) => out.clone(),
            FlipFlop(out, _) => out.clone(),
            Conjunction(out, _) => out.clone(),
            Stub => vec![],
        }
    }

    fn pulses(&mut self, start: usize, b: bool) -> Option<bool> {
        use Module::*;
        match self {
            Broadcaster(_) => Some(b),
            Button(_) => Some(false),
            FlipFlop(_, state) => {
                if !b {
                    *state = !(*state);
                    Some(*state)
                } else {
                    None
                }
            }
            Conjunction(_, memory) => {
                memory.insert(start, b);
                if memory.values().all(|v| *v) {
                    Some(false)
                } else {
                    Some(true)
                }
            }
            Stub => None,
        }
    }
}

fn parse_input(text: &str) -> Result<(Vec<Module>, HashMap<usize, String>)> {
    use Module::*;
    let mut modules = vec![Button(vec![])];
    let mut count: usize = 1;
    let mut module_lookup: HashMap<String, usize> = HashMap::new();
    module_lookup.insert("button".to_string(), 0);
    text.lines().for_each(|l| {
        if let Some((name, outputs)) = l.split_once(" -> ") {
            let clean_name = module_name(name);
            module_lookup.insert(clean_name.clone(), count);
            //println!("{name}: {count}");
            let module = {
                if name == "broadcaster" {
                    modules[0].add_output(count);
                    Broadcaster(vec![])
                } else if name.starts_with('%') {
                    FlipFlop(vec![], false)
                } else if name.starts_with('&') {
                    Conjunction(vec![], HashMap::new())
                } else {
                    panic!("invalid name {name}");
                }
            };
            modules.push(module);
            count += 1;
            outputs.split(", ").for_each(|o| {
                if !module_lookup.contains_key(o) {
                    modules.push(Stub);
                    module_lookup.insert(o.to_string(), count);
                    count += 1;
                }
            });
        } else {
            panic!("{l}");
        }
    });

    text.lines().for_each(|l| {
        if let Some((name, outputs)) = l.split_once(" -> ") {
            let name = module_name(name);
            let module_num = *module_lookup.get(&name).unwrap();

            outputs
                .split(", ")
                .map(|o| module_lookup.get(o).unwrap())
                .for_each(|num| {
                    modules[*num].add_input(module_num);
                    let module = modules.index_mut(module_num);
                    module.add_output(*num)
                });
        }
    });
    let names: HashMap<usize, String> = module_lookup.into_iter().map(|(k, v)| (v, k)).collect();
    Ok((modules, names))
}

fn simulate(modules: &mut Vec<Module>) -> (usize, usize) {
    let mut high = 0;
    let mut low = 0;
    let mut queue = VecDeque::new();
    queue.push_back((0, false, 0));
    while let Some((start, b, end)) = queue.pop_front() {
        let module = modules.index_mut(end);
        let res = module.pulses(start, b);
        if b {
            high += 1;
        } else {
            low += 1;
        }
        if let Some(b) = res {
            module
                .outputs()
                .iter()
                .for_each(|n| queue.push_back((end, b, *n)));
        }
    }
    (high, low - 1)
}

fn module_name(s: &str) -> String {
    if s == "broadcaster" {
        s.to_string()
    } else {
        s[1..s.len()].to_string()
    }
}

fn part1(text: &str) -> Result<()> {
    let (mut modules, _) = parse_input(text)?;
    //println!("{modules:?}");
    let mut high = 0;
    let mut low = 0;
    (0..1000).for_each(|_| {
        let (hc, lc) = simulate(&mut modules);
        high += hc;
        low += lc;
    });
    let res = high * low;
    println!("day 20 part 1: {res}");
    Ok(())
}

/// check whether a low pulse starting at position 0 in `modules` can reach index `low_goal`
fn simulate_part2(modules: &mut Vec<Module>, low_goal: usize) -> bool {
    let mut res = false;
    let mut queue = VecDeque::new();
    queue.push_back((0, false, 0));
    while let Some((start, b, end)) = queue.pop_front() {
        let module = modules.index_mut(end);

        if end == low_goal && !b {
            res = true;
        }
        let res = module.pulses(start, b);
        if let Some(b) = res {
            module
                .outputs()
                .iter()
                .for_each(|n| queue.push_back((end, b, *n)));
        }
    }
    res
}

fn part2(text: &str) -> Result<()> {
    let (modules, names) = parse_input(text).unwrap();
    // I found that (for my input) to reach rx with a low pulse, &df (the only pred of rx) needs to
    // send a low pulse. df has predecessors xl, ln, xp and gp. Those for have each a single
    // predessor, namely zp, pp, sj and rg
    // When investigating the structure of my input I found that all of zp, pp, sj and rg are parts
    // of the circle part of a lasso and in fact each of these lassos is only a simple circle
    // Thus, my solution for part 3 boils down to making sure that all of these 4 are activated at
    // the same time. This requires the lcm (least common multiple) of the circle lengths steps/
    // button presses
    if let Ok((start0, l0)) = pred_lasso(modules.clone(), &names, "zp".to_string(), 100010)
        && let Ok((start1, l1)) = pred_lasso(modules.clone(), &names, "pp".to_string(), 100010)
        && let Ok((start2, l2)) = pred_lasso(modules.clone(), &names, "sj".to_string(), 100010)
        && let Ok((start3, l3)) = pred_lasso(modules, &names, "rg".to_string(), 100010)
    {
        assert_eq!(start0, 0);
        assert_eq!(start1, 0);
        assert_eq!(start2, 0);
        assert_eq!(start3, 0);
        let n1 = lcm(l0, l1);
        let n2 = lcm(n1, l2);
        let res = lcm(n2, l3);
        println!("day 20 part 2: {res}");
    } else {
        println!("Something went wrong in day 20 part 2: Failed to compute lasso lengths!");
    }
    Ok(())
}

/// Return indices of predecessors of `name` in `modules`
/// The button and broadcaster are not included
///
/// * `name`: name of the component
/// * `modules`: all modules
/// * `names`: maps indices in modules to the names of the modules
/// * `include_end`: whether to include the index of `name` in the final result
fn trans_inputs(
    name: String,
    modules: Vec<Module>,
    names: &HashMap<usize, String>,
    include_end: bool,
) -> Result<HashSet<usize>> {
    // inverted names
    let numbers: HashMap<String, usize> = names.iter().map(|(k, v)| (v.clone(), *k)).collect();
    let mut res = HashSet::new();
    // start with number for name
    let num = *numbers.get(&name).unwrap();
    res.insert(num);
    let mut queue = VecDeque::new();
    queue.push_back(num);

    while let Some(n) = queue.pop_front() {
        // indices of direct predecessors of n
        let prev: HashSet<usize> = modules
            .iter()
            .enumerate()
            .filter(|(_, m)| m.outputs().contains(&n))
            .map(|(i, _)| i)
            .collect();
        // newly discovered numbers
        let diff: HashSet<usize> = prev.difference(&res).copied().collect();
        res.extend(diff.clone());
        queue.extend(diff);
    }

    let button_num = *numbers.get("button").unwrap();
    res.remove(&button_num);

    let broadcast_num = *numbers.get("broadcaster").unwrap();
    res.remove(&broadcast_num);
    let mut tmp = res.clone();
    tmp.remove(&num);
    if !include_end {
        res.remove(&num);
    }
    Ok(res)
}

#[derive(Debug)]
struct Lasso<T: Debug> {
    states: Vec<T>,
    start_len: Option<usize>,
}

impl<T: Eq + PartialEq + Debug + Clone> Lasso<T> {
    fn add_state(&mut self, state: T) -> bool {
        for i in 0..self.states.len() {
            if self.states[i] == state {
                self.start_len = Some(i);
                return true;
            }
        }

        self.states.push(state);
        false
    }

    fn cycle_len(&self) -> usize {
        if let Some(l) = self.start_len {
            self.states.len() - l
        } else {
            panic!("")
        }
    }
}

fn pred_lasso(
    modules: Vec<Module>,
    names: &HashMap<usize, String>,
    name: String,
    goal_pos: usize,
) -> Result<(usize, usize)> {
    let name_pred = trans_inputs(name.clone(), modules.clone(), names, false).unwrap();

    let mut lasso: Lasso<String> = Lasso {
        states: vec![],
        start_len: None,
    };

    let mut modules = modules.clone();
    loop {
        let state_desc = name_pred
            .iter()
            .map(|n| (*n, modules[*n].state_description()))
            .sorted()
            .map(|(_, s)| s)
            .join(";");
        if lasso.add_state(state_desc) {
            break;
        }
        if simulate_part2(&mut modules, goal_pos) {
            break;
        }
    }

    let sl = lasso.start_len.unwrap();
    let cl = lasso.cycle_len();

    Ok((sl, cl))
}

pub fn compute() {
    let text = util::read_input_file(20).unwrap();

    let _ = part1(&text);
    let _ = part2(&text);
}
