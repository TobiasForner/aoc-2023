use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use anyhow::Result;
use priority_queue::PriorityQueue;
use random_choice::random_choice;

use crate::util;

#[derive(Clone, Debug)]
struct Graph<T: Clone + Hash + Eq> {
    nodes_map: HashMap<T, usize>,
    /// maps node_id to the id of the main node in its component
    component_map: HashMap<usize, usize>,
    /// maps a node_id to pairs (node_id, weight)
    edges: HashMap<usize, HashMap<usize, usize>>,
}
impl<T: Hash + Eq + Clone + Debug> Graph<T> {
    fn new() -> Self {
        Self {
            nodes_map: HashMap::new(),
            component_map: HashMap::new(),
            edges: HashMap::new(),
        }
    }
    fn add_comp_map(&mut self, src: usize, target: usize) {
        self.component_map.insert(src, target);
    }

    fn add_node_with_id(&mut self, key: T, node_id: usize) {
        self.nodes_map.insert(key, node_id);
    }

    fn get_or_add_node(&mut self, key: T) -> usize {
        let n = self.nodes_map.len();
        if let Some(n) = self.nodes_map.get(&key) {
            *n
        } else {
            self.component_map.insert(n, n);
            self.nodes_map.insert(key, n);
            n
        }
    }

    fn add_edqe(&mut self, key1: T, key2: T) {
        self.add_weighted_edqe(key1, key2, 1);
    }
    fn add_weighted_edqe(&mut self, key1: T, key2: T, weight: usize) {
        let v1 = self.get_or_add_node(key1);
        let v2 = self.get_or_add_node(key2);
        self.edges.entry(v1).or_default().insert(v2, weight);
        self.edges.entry(v2).or_default().insert(v1, weight);
    }

    fn construct_subgraph(&self, nodes: &[usize]) -> Graph<T> {
        let id_to_key: HashMap<usize, T> = self
            .nodes_map
            .iter()
            .map(|(k, id)| (*id, k.clone()))
            .collect();
        let mut g: Graph<T> = Graph::new();
        nodes.iter().for_each(|n1| {
            let k = id_to_key.get(n1).unwrap().clone();
            g.add_node_with_id(k.clone(), *n1);
        });
        nodes.iter().for_each(|n1| {
            let k = id_to_key.get(n1).unwrap().clone();
            if let Some(e) = self.edges.get(n1) {
                e.iter().for_each(|(adj, w)| {
                    let adj_key = id_to_key.get(adj).unwrap().clone();
                    if nodes.contains(adj) {
                        g.add_weighted_edqe(k.clone(), adj_key, *w);
                    }
                });
            }
        });
        self.component_map.keys().for_each(|n| {
            let target = self.resolve_node_mapping(*n);
            if nodes.contains(&target) {
                g.add_comp_map(*n, target);
            }
        });
        g
    }

    fn random_cut(&self) -> (Vec<usize>, Vec<usize>, usize) {
        let mut g = self.clone();
        while g.get_nodes().len() > 2 {
            let edges = g.unique_edges();
            let weights: Vec<f64> = edges.iter().map(|(_, _, w)| *w as f64).collect();
            let choices = random_choice().random_choice_f64(&edges, &weights, 1);
            let (s, t, _) = choices[0];
            g.merge(*s, *t);
            //println!("g after: {g:?}");
        }
        let weight = g
            .unique_edges()
            .iter()
            .map(|(_, _, w)| *w)
            .reduce(|acc, x| acc + x)
            .unwrap();
        let ext_nodes: Vec<Vec<usize>> = g
            .get_nodes()
            .iter()
            .map(|node| g.extended_nodes_for(*node))
            .collect();
        (ext_nodes[0].clone(), ext_nodes[1].clone(), weight)
    }

    fn resolve_node_mapping(&self, node: usize) -> usize {
        let mut node = node;
        while let Some(target) = self.component_map.get(&node)
            && *target != node
        {
            node = *target;
        }
        node
    }

    fn unique_edges(&self) -> Vec<(usize, usize, usize)> {
        let mut res = vec![];
        self.edges.iter().for_each(|(start, endpoints)| {
            endpoints.iter().for_each(|(e, w)| {
                if start < e {
                    res.push((*start, *e, *w));
                }
            });
        });
        res
    }

    fn extended_nodes(&self) -> Vec<usize> {
        self.component_map.keys().cloned().collect()
    }

    fn extended_nodes_for(&self, node_id: usize) -> Vec<usize> {
        self.component_map
            .keys()
            .filter_map(|src| {
                if self.resolve_node_mapping(*src) == node_id {
                    Some(*src)
                } else {
                    None
                }
            })
            .collect()
    }

    fn random_cut_subgraphs(&self) -> (Vec<Graph<T>>, usize) {
        let (s, t, w) = self.random_cut();
        //println!("cut: {s:?}, {t:?}, {w}");
        let mut graphs = vec![];
        let mut get_graphs = |s: &[usize]| {
            let g = self.construct_subgraph(s);
            let mut subgraphs = g.get_connected_graphs();
            graphs.append(&mut subgraphs);
        };
        get_graphs(&s);
        get_graphs(&t);
        (graphs, w)
    }

    /// merges s, t into node id min(s, t)
    fn merge(&mut self, s: usize, t: usize) {
        //println!("start merge {t}->{s}");
        let old_s = s;
        let s = s.min(t);
        let t = t.max(old_s);
        //update edges
        if let Some(e) = self.edges.remove(&t) {
            // store affected endpoints to redirect reverse edges later
            let mut to_change = vec![];
            self.edges.entry(s).and_modify(|es| {
                e.into_iter().for_each(|(end, w)| {
                    if end != s {
                        *es.entry(end).or_insert(0) += w;
                        to_change.push(end);
                    }
                });
                // remove edge to e from s
                let _ = es.remove(&t);
            });
            to_change.iter().for_each(|e| {
                self.edges.entry(*e).and_modify(|es| {
                    if let Some(v) = es.remove(&t) {
                        *es.entry(s).or_insert(0) += v;
                    }
                });
            });
            if let Some(es) = self.edges.get(&s)
                && es.is_empty()
            {
                self.edges.remove(&s);
            }
        }
        //update component_map
        self.component_map.insert(t, s);
    }

    fn get_nodes(&self) -> Vec<usize> {
        self.component_map
            .iter()
            .filter(|(k, v)| k == v)
            .map(|(k, _)| *k)
            .collect()
    }

    fn get_connected_component_of_node(&self, node: usize) -> HashSet<usize> {
        let mut queue = vec![node];
        let mut found = HashSet::new();
        while let Some(n) = queue.pop() {
            found.insert(n);
            if let Some(edges) = self.edges.get(&n) {
                edges.iter().for_each(|(adj, _)| {
                    if !found.contains(adj) {
                        queue.push(*adj);
                    }
                });
            }
        }
        found
    }

    /// returns large_set, s, t (the singleton node in the cut), weight of the cut
    pub fn some_min_s_t_cut(&self) -> (Vec<usize>, usize, usize, usize) {
        let mut nodes: HashSet<usize> = self.get_nodes().into_iter().collect();
        let mut q: PriorityQueue<_, usize> = PriorityQueue::new();
        nodes.iter().for_each(|n| {
            q.push(*n, 0);
        });
        assert!(q.len() > 1);
        let mut picked = HashSet::new();
        let (first, _) = q.pop().unwrap();

        let mut last_picked = first;
        let mut pick = |node: usize, q: &mut PriorityQueue<usize, usize>| {
            nodes.remove(&node);
            picked.insert(node);
            if let Some(adj) = self.edges.get(&node) {
                adj.iter().for_each(|(n, w)| {
                    // q contains all non-picked nodes
                    if let Some((_, p)) = q.get(n) {
                        q.change_priority(n, p + w);
                    }
                });
            }
            last_picked = node;
        };
        pick(first, &mut q);
        while q.len() > 1 {
            let (node, _) = q.pop().unwrap();
            pick(node, &mut q);
        }
        let (node, p) = q.pop().unwrap();
        (picked.into_iter().collect(), last_picked, node, p)
    }

    fn split(&self, s: &[usize], t: &[usize]) -> (Self, Self) {
        (self.construct_subgraph(s), self.construct_subgraph(t))
    }

    fn decompose(&self, k: usize) -> Vec<Self> {
        let mut queue = vec![self.clone()];
        let mut done = vec![];
        while queue.iter().any(|g| !g.edges.is_empty()) {
            let g = queue.pop().unwrap();
            if g.edges.is_empty() {
                assert!(g.get_nodes().len() == 1);
                done.push(g);
                continue;
            }
            let (vs, s, t, w) = g.some_min_s_t_cut();
            if w < k {
                let (g1, g2) = g.split(&vs, &[t]);
                queue.push(g1);
                queue.push(g2);
            } else {
                let mut g = g.clone();
                g.merge(s, t);
                queue.push(g);
            }
        }
        done.append(&mut queue);

        done
    }

    pub fn solve(&self, k: usize) -> Vec<Self> {
        let mut queue = vec![self.clone()];
        let mut done = vec![];
        while let Some(g) = queue.pop() {
            let graphs = g.decompose(k);
            //
            let connected_graphs: Vec<Self> = graphs
                .into_iter()
                .flat_map(|g| g.get_connected_graphs().into_iter())
                .collect();
            if connected_graphs.len() == 1 {
                done.push(connected_graphs[0].clone());
            } else {
                connected_graphs.into_iter().for_each(|g| {
                    queue.push(g);
                });
            }
        }
        done
    }

    fn get_connected_graphs(&self) -> Vec<Graph<T>> {
        let mut found = HashSet::new();
        let mut res = vec![];
        let nodes = self.get_nodes();
        while let Some(n) = nodes.iter().find(|n| !found.contains(*n)) {
            let comp: Vec<usize> = self
                .get_connected_component_of_node(*n)
                .iter()
                .map(|n| {
                    found.insert(*n);

                    *n
                })
                .collect();
            let g = self.construct_subgraph(&comp);
            res.push(g);
        }
        res
    }
}

fn parse_input(text: &str) -> Graph<String> {
    let mut g: Graph<String> = Graph::new();
    text.lines().for_each(|l| {
        let (src, ends) = l.split_once(": ").unwrap();
        ends.split(' ').for_each(|e| {
            g.add_edqe(src.to_string(), e.to_string());
        });
    });
    g
}

fn part1_random(text: &str) -> usize {
    let g = parse_input(text);
    loop {
        let (graphs, w) = g.random_cut_subgraphs();
        if graphs.len() == 2 && w == 3 {
            break graphs[0].get_nodes().len() * graphs[1].get_nodes().len();
        }
    }
}
fn part1_decompose(text: &str) -> usize {
    let g = parse_input(text);
    let parts = g.solve(4);
    parts[0].extended_nodes().len() * parts[1].extended_nodes().len()
}

fn part1(text: &str) -> Result<()> {
    let res = part1_random(text);
    println!("part 1 (random): {res}");

    let res = part1_decompose(text);
    println!("part 1 (decompose): {res}");
    Ok(())
}

pub fn compute() {
    let text = util::read_input_file(25).unwrap();
    let _ = part1(&text);
}
#[test]
fn test_example() {
    let text = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
    assert_eq!(part1_random(text), 54);
}
