use anyhow::{bail, Context, Result};
use itertools::{Diff, Itertools};

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
    FlipFlop(Vec<usize>, bool),                           //output,state
    Conjunction(Vec<usize>, HashMap<usize, bool>, usize), //output,memory,input_count
    Stub,
}

impl Module {
    fn add_output(&mut self, output: usize) {
        use Module::*;
        match self {
            Broadcaster(out) => out.push(output),
            Button(out) => out.push(output),
            FlipFlop(out, _) => out.push(output),
            Conjunction(out, _, _) => out.push(output),
            Stub => {}
        }
    }

    fn state_description(&self) -> String {
        use Module::*;
        let empty = "".to_string();
        match self {
            Broadcaster(out) => empty,
            Button(out) => empty,
            FlipFlop(out, state) => {
                if *state {
                    "t".to_string()
                } else {
                    "f".to_string()
                }
            }
            Conjunction(_, memory, _) => memory
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
            FlipFlop(out, _) => {}
            Conjunction(out, memory, count) => {
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
            Conjunction(out, _, _) => out.clone(),
            Stub => vec![],
        }
    }

    fn out_count(&self) -> usize {
        use Module::*;
        match self {
            Broadcaster(out) => out.len(),
            Button(out) => out.len(),
            FlipFlop(out, _) => out.len(),
            Conjunction(_, out, _) => out.len(),
            Stub => 0,
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
            Conjunction(inp_, memory, count) => {
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

    fn variant(&self) -> char {
        match self {
            Self::Broadcaster(_) => 'B',
            Self::Button(_) => 'I',
            Self::Conjunction(_, _, _) => 'C',
            Self::FlipFlop(_, _) => 'F',
            Self::Stub => 'S',
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
            println!("{name}: {count}");
            let module = {
                if name == "broadcaster" {
                    modules[0].add_output(count);
                    Broadcaster(vec![])
                } else if name.starts_with('%') {
                    FlipFlop(vec![], false)
                } else if name.starts_with('&') {
                    Conjunction(vec![], HashMap::new(), 0)
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

fn simulate(modules: &mut Vec<Module>, names: &HashMap<usize, String>) -> (usize, usize) {
    let mut high = 0;
    let mut low = 0;
    let mut queue = VecDeque::new();
    queue.push_back((0, false, 0));
    while let Some((start, b, end)) = queue.pop_front() {
        let module = modules.index_mut(end);
        //let def = "default".to_string();
        //let start_name = names.get(&start).unwrap_or(&def);
        //let def = "default".to_string();
        //let end_name = names.get(&end).unwrap_or(&def);
        let res = module.pulses(start, b);
        //println!("{start_name} -{b}-> {end_name}");
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
    let (mut modules, names) = parse_input(text)?;
    println!("{modules:?}");
    let mut high = 0;
    let mut low = 0;
    (0..1000).for_each(|i| {
        //println!("iter {i}");
        let (hc, lc) = simulate(&mut modules, &names);
        //println!("high: {hc}; low: {lc}");
        high += hc;
        low += lc;
    });
    let res = high * low;
    println!("day 20 part 1: {res}");
    Ok(())
}

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
        //println!("{start_name} -{b}-> {end_name}");
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
    let (mut modules, names) = parse_input(text)?;
    let mut res: usize = 0;
    let low_goal = (0..modules.len())
        .filter_map(|i| {
            if names.contains_key(&i) && names.get(&i).unwrap() == "xl" {
                Some(i)
            } else {
                None
            }
        })
        .next()
        .unwrap();
    loop {
        if res % 1000 == 0 {
            println!("{res}");
        }
        res += 1;
        if simulate_part2(&mut modules, low_goal) {
            break;
        }
    }
    println!("part 2: {res}");
    Ok(())
}

fn trans_inputs(
    name: String,
    modules: Vec<Module>,
    names: &HashMap<usize, String>,
    include_end: bool,
) -> Result<HashSet<usize>> {
    let numbers: HashMap<String, usize> =
        names.iter().map(|(k, v)| (v.clone(), k.clone())).collect();
    let mut res = HashSet::new();
    let num = *numbers.get(&name).unwrap();
    res.insert(num);
    let mut queue = VecDeque::new();
    queue.push_back(num);

    while let Some(n) = queue.pop_front() {
        let prev: HashSet<usize> = modules
            .iter()
            .enumerate()
            .filter(|(_, m)| m.outputs().iter().any(|o| *o == n))
            .map(|(i, _)| i)
            .collect();
        let diff1: HashSet<usize> = prev.difference(&res).map(|n| *n).collect();
        res.extend(diff1.clone());
        queue.extend(diff1);
    }
    println!("{res:?}");

    let button_num = *numbers.get("button").unwrap();
    res.remove(&button_num);

    let broadcast_num = *numbers.get("broadcaster").unwrap();
    res.remove(&broadcast_num);
    let mut tmp = res.clone();
    tmp.remove(&num);
    let vars: HashSet<char> = tmp.iter().map(|n| modules[*n].variant()).collect();
    println!("{vars:?}");
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
) -> Result<usize> {
    let t1 = trans_inputs(name.clone(), modules.clone(), names, false).unwrap();

    let mut l1: Lasso<String> = Lasso {
        states: vec![],
        start_len: None,
    };

    let mut res: usize = 0;
    /*let rx_pos = (0..modules.len())
    .filter_map(|i| {
        if names.contains_key(&i) && names.get(&i).unwrap() == "rx" {
            Some(i)
        } else {
            None
        }
    })
    .next()
    .unwrap();*/
    let mut modules = modules.clone();
    loop {
        let state_desc = t1
            .iter()
            .map(|n| (*n, modules[*n].state_description()))
            .sorted()
            .map(|(_, s)| s)
            .join(";");
        if l1.add_state(state_desc) {
            break;
        }
        if res % 1000 == 0 {
            println!("{res}");
        }
        res += 1;
        if simulate_part2(&mut modules, goal_pos) {
            break;
        }
    }

    let sl = l1.start_len.unwrap();
    let cl = l1.cycle_len();

    println!("{name}: {sl} {cl}");

    Ok(res)
}

fn lassos(text: &str) {
    let (modules, names) = parse_input(text).unwrap();
    let res = pred_lasso(modules.clone(), &names, "zp".to_string(), 100010);
    println!("{res:?}");

    let res = pred_lasso(modules.clone(), &names, "pp".to_string(), 100010);
    println!("{res:?}");

    let res = pred_lasso(modules.clone(), &names, "sj".to_string(), 100010);
    println!("{res:?}");

    let res = pred_lasso(modules, &names, "rg".to_string(), 100010);
    println!("{res:?}");
}

pub fn compute() {
    let text = util::read_input_file(20).unwrap();
    lassos(&text);

    //let _ = part1(&text);
    //let _ = part2(&text);

    let n1 = lcm(4051, 4021);
    let n2 = lcm(n1, 4057);
    let res = lcm(n2, 3833);
    println!("{res}");
}
