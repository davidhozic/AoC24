use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::print_map;


pub fn part_one() {
    let map = parse_input();
    let graph = graph_from_map(&map);
    let (path, score) = get_shortest_path(
        &graph,
        (map.len() - 2, 1usize, Direction::EAST),
        (1, map[0].len() - 2)
    );

    let mut dir = Direction::EAST;
    let mut last_y = path[0].0;
    let mut last_x = path[0].1;
    let mut score_c = 0;
    for (y, x) in &path[1..] {
        match dir {
            Direction::EAST => {
                if *x != last_x + 1 {
                    if *y == last_y + 1 {
                        dir = Direction::SOUTH;
                    }                    
                    else {
                        dir = Direction::NORTH;
                    }
                    score_c += 1000;
                }
            },
            Direction::WEST => {
                if *x != last_x - 1 {
                    if *y == last_y + 1 {
                        dir = Direction::SOUTH;
                    }                    
                    else {
                        dir = Direction::NORTH;
                    }
                    score_c += 1000;
                }
            },
            Direction::SOUTH => {
                if *y != last_y + 1 {
                    if *x == last_x + 1 {
                        dir = Direction::EAST;
                    }                    
                    else {
                        dir = Direction::WEST;
                    }
                    score_c += 1000;
                }
            },
            Direction::NORTH => {
                if *y != last_y - 1 {
                    if *x == last_x + 1 {
                        dir = Direction::EAST;
                    }                    
                    else {
                        dir = Direction::WEST;
                    }
                    score_c += 1000;
                }
            },
        }

        last_x = *x;
        last_y = *y;
        score_c += 1;
    }

    print_map(&map);
    println!("{score} {score_c} {}", path.len());
}


pub fn part_two() {

}

pub fn parse_input() -> Vec<Vec<char>> {
    let input: String = read_to_string("./inputs/day16.txt").unwrap();
    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    map
}


/// Finds the best path using Dijkstra's algorithm. Returns the path and the score using this path.
fn get_shortest_path(
    graph: &HashMap<(usize, usize, Direction), Vec<(usize, usize, Direction, usize)>>,
    start_node: (usize, usize, Direction),
    goal_node: (usize, usize)
) -> (Vec<(usize, usize)>, usize) {
    let mut ol = HashMap::new();
    let mut cl = HashMap::new();
    ol.insert(start_node, (0, (0, 0, Direction::EAST)));
    while !ol.is_empty() {
        // Select current best
        let (node, (cost, parent)) = {  // Gotta love Rust's lifetimes :)
            let n: (&(usize, usize, Direction), &(usize, (usize, usize, Direction))) = ol.iter().min_by_key(|(_, v)| v.0).unwrap();
            (*n.0, *n.1)
        };

        // Update neigbors
        // if let None = graph.get(&node) {
        //     panic!("{node:?}");
        // }

        for (ny, nx, direction, edge_price) in graph.get(&node).unwrap_or(&Vec::with_capacity(0)) {
            if cl.contains_key(&(*ny, *nx, *direction)) {
                continue;
            }

            // Add to open list if the new cost is better
            let new_cost = edge_price + cost;
            if let Some((neighbor_cost, _)) = ol.get(&(*ny, *nx, *direction)) {
                if new_cost < *neighbor_cost {
                    ol.insert((*ny, *nx, *direction), (new_cost, node));    
                }
            }
            else {
                ol.insert((*ny, *nx, *direction), (new_cost, node));
            }
        }

        cl.insert(node, (cost, parent));
        ol.remove(&node);
        // if node == goal_node {  // Found best path to the end
        //     break;
        // }
    }


    // Create the path
    let mut path = vec![goal_node];
    let start = cl.iter().filter(|(k, _)| (k.0, k.1) == goal_node)
        .min_by_key(|(_, v)| v.0).unwrap().1;

    let (best_cost, mut parent) = start;
    while parent != start_node {
        path.push((parent.0, parent.1));
        parent = cl[&parent].1
    }

    path.push((start_node.0, start_node.1));
    (path.iter().rev().map(|x| *x).collect(), *best_cost)
}


fn graph_from_map(map: &Vec<Vec<char>>) -> HashMap<(usize, usize, Direction), Vec<(usize, usize, Direction, usize)>> {
    let mut positions= vec![(map.len() - 2, 1usize, Direction::EAST)];  // Tracks current position in the graph
    let mut nodes = HashMap::new();

    let mut visited = HashSet::new();
    // Find nodes and edges of the map. This basically builds a graph.
    while positions.len() > 0 {  // Iterate until all cursors leave the map
        // Check neigbors
        let mut new_positions = Vec::new();
        for (y, x, direction) in &positions {
            if visited.contains(&(*y, *x, *direction)) {
                continue;
            }

            for new_direction in Direction::iter() {
                let (yn, xn) = match new_direction {
                    Direction::NORTH => (*y - 1, *x),
                    Direction::SOUTH => (*y + 1, *x),
                    Direction::WEST => (*y, *x - 1),
                    Direction::EAST => (*y, *x + 1),
                };

                let delta_direction = match direction {
                    Direction::NORTH if new_direction == Direction::SOUTH => continue,
                    Direction::SOUTH if new_direction == Direction::NORTH => continue,
                    Direction::EAST if new_direction == Direction::WEST => continue,
                    Direction::WEST if new_direction == Direction::EAST => continue,
                    _ if *direction != new_direction => 1000,
                    _  => 0,
                };
                
                if map[yn][xn] == '#' {  // Reached a wall || Already visisted
                    continue;
                }

                let edge_cost = delta_direction + 1;  // Rotation = 1000 points, move = 1 point
                new_positions.push((yn, xn, new_direction));
                let edge_list = nodes.entry((*y, *x, *direction)).or_insert(Vec::new());
                edge_list.push((yn, xn, new_direction, edge_cost));
            }
            visited.insert((*y, *x, *direction));
        }
        positions = new_positions;
    }

    nodes
}


#[derive(Debug, EnumIter, PartialEq, Clone, Copy, Eq, Hash)]
enum Direction {
    NORTH = 0,
    WEST = 1,
    SOUTH = 2,
    EAST = 3
}
