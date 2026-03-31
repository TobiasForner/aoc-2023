use anyhow::Result;
use itertools::Itertools;

use crate::util::{self, Direction, Graph};

use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

pub trait DomGen {
    fn dom_gen(&self) -> Vec<Self>
    where
        Self: Sized;

    fn dom_gen_part2(&self) -> Vec<Self>
    where
        Self: Sized;
}

impl<N: DomGen + Hash + Eq + Clone + Debug> Graph<N> {
    fn dijkstra(&self, source: N, part2: bool) -> HashMap<N, usize> {
        let mut unvisited: HashSet<N> = self.edges.keys().cloned().collect();
        let mut distances = HashMap::new();
        distances.insert(source.clone(), 0);
        let mut current = source;
        loop {
            let len = unvisited.len();
            //println!("{len}");
            if len.is_multiple_of(1000) {
                println!("{len}");
            }
            let cdist = *distances.get(&current).unwrap_or(&usize::MAX);
            if let Some(neigh) = self.edges.get(&current) {
                neigh.iter().for_each(|(n, w)| {
                    if unvisited.contains(n) {
                        let n = n.clone();
                        let dist = *distances.get(&n).unwrap_or(&usize::MAX);
                        let new_dist = cdist.saturating_add(*w);
                        if new_dist < dist {
                            //println!("dist {n:?}: {new_dist}");
                            distances.insert(n.clone(), new_dist);
                        }
                    }
                });
            }
            if part2 {
                current.dom_gen_part2().iter().for_each(|d| {
                    unvisited.remove(d);
                });
            } else {
                current.dom_gen().iter().for_each(|d| {
                    unvisited.remove(d);
                });
            }

            let min_dist = unvisited
                .iter()
                .map(|n| (n, *distances.get(n).unwrap_or(&usize::MAX)))
                .min_by_key(|(_, d)| *d);

            if let Some((c, d)) = min_dist {
                if d < usize::MAX {
                    current = c.clone();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        distances
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Node {
    x: usize,
    y: usize,
    direction: Direction,
    count: usize,
}

impl DomGen for Node {
    fn dom_gen(&self) -> Vec<Self> {
        (self.count..=3)
            .map(|c| Self {
                x: self.x,
                y: self.y,
                direction: self.direction.clone(),
                count: c,
            })
            .collect_vec()
    }

    fn dom_gen_part2(&self) -> Vec<Self> {
        (self.count..=10)
            .map(|c| Self {
                x: self.x,
                y: self.y,
                direction: self.direction.clone(),
                count: c,
            })
            .collect_vec()
    }
}

fn parse_graph(text: &str) -> Graph<Node> {
    use Direction::*;
    let dir_pairs = [
        (Left, Left),
        (Left, Up),
        (Left, Down),
        (Right, Right),
        (Right, Up),
        (Right, Down),
        (Up, Left),
        (Up, Right),
        (Up, Up),
        (Down, Left),
        (Down, Right),
        (Down, Down),
    ];
    let mut res = Graph::new();
    text.lines().enumerate().for_each(|(y, l)| {
        let height = text.lines().count();
        let width = text.lines().next().expect("msg").len();
        l.chars().enumerate().for_each(|(x, _)| {
            // handle start
            let s = if x == 0 && y == 0 { 0 } else { 1 };
            (s..=3).for_each(|c| {
                dir_pairs.iter().for_each(|(d1, d2)| {
                    if d1 != d2 || c < 3 {
                        let (endx, endy) = move_in_dir(x, y, d2);

                        if (endx != x || endy != y) && (endx < width && endy < height) {
                            let start = Node {
                                x,
                                y,
                                direction: d1.clone(),
                                count: c,
                            };
                            let count = { if d1 == d2 { c + 1 } else { 1 } };
                            let end = Node {
                                x: endx,
                                y: endy,
                                direction: d2.clone(),
                                count,
                            };
                            let weight = text
                                .lines()
                                .nth(endy)
                                .expect("")
                                .chars()
                                .nth(endx)
                                .unwrap_or_else(|| panic!("Can't get {endx}th from length {width}"))
                                .to_digit(10)
                                .expect("Can't parse char")
                                as usize;
                            res.add_directed_edge(start, end, weight)
                        }
                    }
                });
            })
        })
    });
    res
}

fn parse_graph_part2(text: &str) -> Graph<Node> {
    let grid: Vec<Vec<usize>> = text
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    use Direction::*;
    let dir_pairs = [
        (Left, Left),
        (Left, Up),
        (Left, Down),
        (Right, Right),
        (Right, Up),
        (Right, Down),
        (Up, Left),
        (Up, Right),
        (Up, Up),
        (Down, Left),
        (Down, Right),
        (Down, Down),
    ];
    let mut res = Graph::new();
    text.lines().enumerate().for_each(|(y, l)| {
        let height = text.lines().count();
        let width = text.lines().next().expect("msg").len();
        l.chars().enumerate().for_each(|(x, _)| {
            // handle start
            if x == 0 && y == 0 {
                let start = Node {
                    x,
                    y,
                    direction: Down,
                    count: 0,
                };
                let end1 = advance_by(&start, 4, &Down).unwrap();
                let weight1 = path_weight(&start, 4, &Down, &grid);
                res.add_directed_edge(start.clone(), end1, weight1);
                let end2 = advance_by(&start, 4, &Right).unwrap();
                let weight1 = path_weight(&start, 4, &Right, &grid);
                res.add_directed_edge(start, end2, weight1);
            }
            (4..=10).for_each(|c| {
                dir_pairs.iter().for_each(|(d1, d2)| {
                    let start = Node {
                        x,
                        y,
                        direction: d1.clone(),
                        count: c,
                    };
                    let dist = { if d1 == d2 { 1 } else { 4 } };
                    if let Some(end) = advance_by(&start, dist, d2)
                        && end.count <= 10
                        && end.x < width
                        && end.y < height
                    {
                        let weight = path_weight(&start, dist, d2, &grid);
                        res.add_directed_edge(start, end, weight);
                    }
                });
            })
        })
    });
    res
}

fn path_weight(start: &Node, dist: usize, direction: &Direction, grid: &[Vec<usize>]) -> usize {
    use Direction::*;
    let x = start.x;
    let y = start.y;
    (1..=dist)
        .map(|p| match direction {
            Up => grid[y - p][x],
            Down => grid[y + p][x],
            Left => grid[y][x - p],
            Right => grid[y][x + p],
        })
        .sum()
}

fn advance_by(n: &Node, dist: usize, direction: &Direction) -> Option<Node> {
    let d2 = direction.clone();

    let (endx, endy) = move_n_in_dir(n.x, n.y, &d2, dist);

    if n.x.abs_diff(endx) == dist || n.y.abs_diff(endy) == dist {
        let count = {
            if n.direction == *direction {
                n.count + dist
            } else {
                dist
            }
        };
        let end = Node {
            x: endx,
            y: endy,
            direction: d2.clone(),
            count,
        };
        return Some(end);
    }

    None
}

fn move_n_in_dir(x: usize, y: usize, dir: &Direction, n: usize) -> (usize, usize) {
    use Direction::*;
    match dir {
        Left => (x.saturating_sub(n), y),
        Right => (x + n, y),
        Up => (x, y.saturating_sub(n)),
        Down => (x, y + n),
    }
}

fn move_in_dir(x: usize, y: usize, dir: &Direction) -> (usize, usize) {
    move_n_in_dir(x, y, dir, 1)
}

fn part1(text: &str) -> Result<()> {
    let height = text.lines().count();
    let width = text.lines().next().expect("msg").len();
    let graph = parse_graph(text);
    //println!("{graph:?}");
    let source = Node {
        x: 0,
        y: 0,
        direction: Direction::Down,
        count: 0,
    };
    let distances = graph.dijkstra(source, false);
    //println!("{distances:?}");
    let res = distances
        .into_iter()
        .filter(|(n, _)| n.x == width - 1 && n.y == height - 1)
        .map(|(_, d)| d)
        .min()
        .expect("msg");
    println!("part 1: {res}");
    Ok(())
}

fn part2(text: &str) -> Result<()> {
    let height = text.lines().count();
    let width = text.lines().next().expect("msg").len();
    let graph = parse_graph_part2(text);
    //println!("{graph:?}");
    let source = Node {
        x: 0,
        y: 0,
        direction: Direction::Down,
        count: 0,
    };
    let distances = graph.dijkstra(source, true);
    //println!("{distances:?}");
    let res = distances
        .into_iter()
        .filter(|(n, _)| n.x == width - 1 && n.y == height - 1)
        .map(|(_, d)| d)
        .min()
        .expect("msg");
    println!("part 2: {res}");
    Ok(())
}

pub fn compute() {
    let text = util::read_input_file(17).unwrap();
    let _ = part1(&text);
    let _ = part2(&text);
}
