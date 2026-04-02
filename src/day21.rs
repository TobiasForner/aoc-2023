use anyhow::{Context, Result};
use priority_queue::PriorityQueue;

use std::{cmp::Reverse, collections::HashSet};

fn parse_input(text: &str) -> Result<(Vec<Vec<char>>, usize, usize)> {
    let map: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();
    let (start_x, start_y): (usize, usize) = map
        .iter()
        .enumerate()
        .find_map(|(index, v)| {
            v.iter()
                .enumerate()
                .find_map(|(i, x)| if *x == 'S' { Some((i, index)) } else { None })
        })
        .context("")?;
    Ok((map, start_x, start_y))
}

fn part1(text: &str) -> Result<()> {
    let (map, start_x, start_y) = parse_input(text)?;
    let distances = dijkstra(start_x, start_y, &map);
    let res = distances.reachable_in_steps(64);
    println!("part 1: {res}");
    Ok(())
}

fn part2(text: &str, steps: i64) -> Result<()> {
    let res = part2_comp(text, steps)?;
    println!("part 2: {res}");
    Ok(())
}

fn part2_comp(text: &str, steps: i64) -> Result<usize> {
    let (map, start_x, start_y) = parse_input(text)?;
    // compute distances once for the relevant entry points
    let dij_center = dijkstra(start_x, start_y, &map);
    let dij_right_center = dijkstra(map[0].len() - 1, start_y, &map);
    let dij_top_center = dijkstra(start_x, 0, &map);
    let dij_bottom_center = dijkstra(start_x, map.len() - 1, &map);
    let dij_left_center = dijkstra(0, start_y, &map);
    let dij_bottom_right = dijkstra(map[0].len() - 1, map.len() - 1, &map);
    let dij_top_right = dijkstra(map[0].len() - 1, 0, &map);
    let dij_bottom_left = dijkstra(0, map.len() - 1, &map);
    let dij_top_left = dijkstra(0, 0, &map);

    // center: starting pattern, starting at the starting position
    let mut res = dij_center.reachable_in_steps(steps as usize);

    // top center column
    let mut steps_top = steps - start_y as i64 - 1;
    while steps_top >= 0 {
        res += dij_bottom_center.reachable_in_steps(steps_top as usize);
        steps_top -= map.len() as i64;
    }

    // bottom center column
    let mut steps_bottom = steps + start_y as i64 - map.len() as i64;
    while steps_bottom >= 0 {
        res += dij_top_center.reachable_in_steps(steps_bottom as usize);
        steps_bottom -= map.len() as i64;
    }
    println!("center column done");

    // left half
    // we go left from the starting position until we reach the right edge of the first pattern left
    // of the starting pattern. We handle that column and then walk to the right edge of the next
    // pattern,...
    let mut steps_left = steps - (start_x as i64) - 1;
    while steps_left >= 0 {
        // first patten
        res += dij_right_center.reachable_in_steps(steps_left as usize);

        // patterns in the top part of the current column
        let steps_top = steps_left - start_y as i64 - 1;
        res += visitable_in_direction(steps_top, &dij_bottom_right);

        // patterns in the bottom part of the current column
        let steps_bottom = steps_left + start_y as i64 - map.len() as i64;
        res += visitable_in_direction(steps_bottom, &dij_top_right);
        steps_left -= map[0].len() as i64;
    }
    println!("left half done");

    // right half: now we start our patterns on the left edge
    let mut steps_right = steps + (start_x as i64) - map[0].len() as i64;
    while steps_right >= 0 {
        res += dij_left_center.reachable_in_steps(steps_right as usize);

        // patterns in the top part of the current column
        let steps_top = steps_right - start_y as i64 - 1;
        res += visitable_in_direction(steps_top, &dij_bottom_left);

        // patterns in the bottom part of the current column
        let steps_bottom = steps_right + start_y as i64 - map.len() as i64;
        res += visitable_in_direction(steps_bottom, &dij_top_left);

        steps_right -= map[0].len() as i64;
    }

    Ok(res)
}

/// returns (x,y) coordinates
/// map should be defined in terms of (y,x) coordinates
fn get_map_neighbors(x: usize, y: usize, map: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let mut positions = vec![];
    if x > 0 {
        positions.push((x - 1, y));
    }
    if y > 0 {
        positions.push((x, y - 1));
    }
    if x + 1 < map[0].len() {
        positions.push((x + 1, y));
    }
    if y + 1 < map.len() {
        positions.push((x, y + 1));
    }
    positions
        .into_iter()
        .filter(|(x, y)| map[*y][*x] == '.' || map[*y][*x] == 'S')
        .collect()
}

/// returns the number of unique locations that can be reached using exactly `steps` many steps in
/// one vertical direction.
/// The actual movement is abstracted away by using a Distances struct.
/// This function uses the fact that (at least in my input), there are unobstructed lines from the
/// starting point in all 4 directions.
fn visitable_in_direction(steps: i64, distances: &Distances) -> usize {
    let length = distances.distances.len() as i64;
    let mut steps = steps;
    let mut res = 0;
    // first we do a "compressed" computation of the number of locations that belong to patterns in
    // which we know that all possible locations can be visited.
    // Note that for non-trivial patterns, we can never visit all locations as we need to use
    // exactly all the steps. Thus, depending on whether we have an even or odd number of steps
    // left, we can only ever reach a part of the locations in the pattern

    // compute the maximum number of steps required from the starting point to reach all possible locations
    // The Distances struct contains helper valuas for this
    let max_needed_for_all = distances.max_even_dist.max(distances.max_odd_dist);
    // to simplify the computation, we simply deduct this max step number from the overall number of
    // steps required. Then, we know that in all patterns we can reach at all, we will be able to
    // reach all possible locations. The possible remainder is handled later.
    let reduced_steps = steps - max_needed_for_all as i64;
    if reduced_steps >= 0 {
        let num_max_visits = reduced_steps / length; //number of patterns in which we definitely can
        //reach the maximum possible number of locations

        //if the length of the pattern is odd, about half of the patterns are reached with odd steps
        //left and with even steps left, hence we need to check a few cases based on this and wheter
        //`steps` is odd or even
        if length % 2 == 1 {
            let half = (num_max_visits / 2) as usize;
            if num_max_visits % 2 == 0 {
                res += half * distances.even_count;
                res += half * distances.odd_count;
            } else if steps % 2 == 1 {
                res += half * distances.even_count;
                res += (half + 1) * distances.odd_count;
            } else {
                res += (half + 1) * distances.even_count;
                res += half * distances.odd_count;
            }
        } else if steps % 2 == 0 {
            res += num_max_visits as usize * distances.even_count;
        } else {
            res += num_max_visits as usize * distances.odd_count;
        }
        steps -= num_max_visits * length;
    }

    // handle the remaining steps for patterns that are potentially not maximally explorable
    while steps >= 0 {
        res += distances.reachable_in_steps(steps as usize);
        steps -= length;
    }
    res
}

#[derive(Debug)]
struct Distances {
    distances: Vec<Vec<Option<usize>>>,
    max_even_dist: usize,
    even_count: usize,
    max_odd_dist: usize,
    odd_count: usize,
}
impl Distances {
    fn new(distances: Vec<Vec<Option<usize>>>) -> Self {
        let mut even_count = 0;
        let mut max_even_dist = 0;
        let mut odd_count = 0;
        let mut max_odd_dist = 0;
        distances.iter().for_each(|v| {
            v.iter().for_each(|x| {
                if let Some(x) = x
                    && *x < 999999999999
                {
                    if x % 2 == 0 {
                        even_count += 1;
                        max_even_dist = max_even_dist.max(*x);
                    } else {
                        odd_count += 1;
                        max_odd_dist = max_odd_dist.max(*x);
                    }
                }
            })
        });
        Self {
            distances,
            max_even_dist,
            even_count,
            max_odd_dist,
            odd_count,
        }
    }

    /// returns the number of distinct locations that can be reached using a number of steps equal to `steps`
    fn reachable_in_steps(&self, steps: usize) -> usize {
        if steps.is_multiple_of(2) && steps >= self.max_even_dist {
            self.even_count
        } else if steps % 2 == 1 && steps >= self.max_odd_dist {
            self.odd_count
        } else {
            self.distances
                .iter()
                .flat_map(|v| {
                    v.iter().filter_map(|d| {
                        if let Some(d) = d {
                            if *d <= steps && *d % 2 == steps % 2 {
                                Some(1)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                })
                .reduce(|a, b| a + b)
                .unwrap()
        }
    }
}

fn dijkstra(start_x: usize, start_y: usize, map: &[Vec<char>]) -> Distances {
    // priority queue mapping (x,y) to the currently known mindist to start
    let mut pq: PriorityQueue<_, Reverse<usize>> = PriorityQueue::new();
    // distances[y][x] is the minimum distance from (start_x, start_y) to (x,y) discovered so far
    let mut distances: Vec<Vec<Option<usize>>> = map
        .iter()
        .enumerate()
        .map(|(y, v)| {
            v.iter()
                .enumerate()
                .map(|(x, _)| {
                    if map[y][x] == '#' {
                        None
                    } else {
                        Some(if x == start_x && y == start_y {
                            let prio: usize = 0;
                            pq.push((x, y), Reverse(prio));
                            prio
                        } else {
                            let prio = 999999999999;
                            pq.push((x, y), Reverse(prio));
                            prio
                        })
                    }
                })
                .collect()
        })
        .collect();
    while let Some(((x, y), d)) = pq.pop() {
        let new_dist = d.0 + 1;
        get_map_neighbors(x, y, map).iter().for_each(|(a, b)| {
            if let Some(d) = distances[*b][*a]
                && new_dist < d
            {
                distances[*b][*a] = Some(new_dist);
                if pq.change_priority(&(*a, *b), Reverse(new_dist)).is_none() {
                    pq.push((*a, *b), Reverse(new_dist));
                }
            }
        });
    }
    Distances::new(distances)
}

/// utility function used for testing my implementation
fn _brute_force_part2(text: &str, steps: i64) -> Result<usize> {
    let (map, start_x, start_y) = parse_input(text)?;
    // expand map steps number of times in each direction
    // I do this by extending the map downward by 2*steps copies and then the same to the right
    // then we simply need to adjust the starting position

    let mut extended_map = map.clone();
    (0..(2 * steps)).for_each(|_| map.iter().for_each(|v| extended_map.push(v.clone())));
    let ext_map = extended_map.clone();
    extended_map.iter_mut().enumerate().for_each(|(y, v)| {
        (0..(2 * steps)).for_each(|_| {
            ext_map[y].iter().for_each(|x| v.push(*x));
        });
    });
    let start_x = start_x + steps as usize * map[0].len();
    let start_y = start_y + steps as usize * map.len();
    println!(
        "extended map: {}x{}",
        extended_map.len(),
        extended_map[0].len()
    );
    println!("start_x: {start_x}");
    println!("start_y: {start_y}");
    let distances = dijkstra(start_x, start_y, &extended_map);
    //println!("distances: {distances:?}");
    Ok(distances.reachable_in_steps(steps as usize))
}

pub fn compute() {
    let text = fs::read_to_string("inputs/day21.txt").expect("expected readable file");
    let _ = part1(&text);
    let _ = part2(&text, 26501365);
}

#[test]
fn test_day21_part2() {
    let text = "...\n.S.\n...";
    let res = part2_comp(text, 5);
    assert_eq!(res.unwrap(), 36);
    assert_eq!(_brute_force_part2(text, 5).unwrap(), 36);

    let text = "...\n.S.\n...";
    let steps = 20;
    assert_eq!(
        _brute_force_part2(text, steps).unwrap(),
        part2_comp(text, steps).unwrap()
    );

    let text = "....\n.S..\n..#.\n....";
    let steps = 10;
    assert_eq!(
        _brute_force_part2(text, steps).unwrap(),
        part2_comp(text, steps).unwrap()
    );
    let text = "....\n..#.\n.S..\n....";
    let steps = 10;
    assert_eq!(
        _brute_force_part2(text, steps).unwrap(),
        part2_comp(text, steps).unwrap()
    );

    let text = "....\n..#.\n.S..\n..#.\n....";
    let steps = 10;
    assert_eq!(
        _brute_force_part2(text, steps).unwrap(),
        part2_comp(text, steps).unwrap()
    );
}

#[test]
fn test_brute_force() {
    let text = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    assert_eq!(_brute_force_part2(text, 6).unwrap(), 16);
    assert_eq!(_brute_force_part2(text, 10).unwrap(), 50);
    assert_eq!(_brute_force_part2(text, 50).unwrap(), 1594);
}
