use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
};

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day23_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = longest_hike(&input, true);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = longest_hike(&input, false);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

type Node = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    SlopeUp,
    SlopeDown,
    SlopeLeft,
    SlopeRight,
}

fn longest_hike(input: &str, slippery_slopes: bool) -> Result<usize, String> {
    let trail = process_trail(input, slippery_slopes)?;

    let mut graph = BTreeMap::new();
    make_into_graph(
        &trail,
        (1, 0),
        (1, 1),
        (trail[0].len() - 2, trail.len() - 1),
        &mut graph,
    );

    walk_trail(
        &graph,
        (1, 0),
        (trail[0].len() - 2, trail.len() - 1),
        BTreeSet::new(),
    )
    .ok_or("Failed to calculate stroll path.".to_owned())
}

fn make_into_graph(
    trail: &[Vec<Tile>],
    start: Node,
    next: Node,
    end: Node,
    graph: &mut BTreeMap<Node, BTreeSet<(Node, usize)>>,
) {
    let next_junction = next_junction(trail, start, next, end);
    graph
        .entry(start)
        .and_modify(|set| {
            set.insert(next_junction);
        })
        .or_insert_with(|| BTreeSet::from_iter([next_junction]));
    if next_junction.0 != end && !graph.contains_key(&next_junction.0) {
        let directions = collect_directions(trail, next_junction.0, end, &BTreeSet::new());
        for new_dir in directions {
            make_into_graph(trail, next_junction.0, new_dir, end, graph)
        }
    }
}

fn next_junction(trail: &[Vec<Tile>], start: Node, mut next: Node, end: Node) -> (Node, usize) {
    let mut path = BTreeSet::from_iter([start]);
    let mut steps = 1;
    while let [single_dir] = collect_directions(trail, next, end, &path).as_slice() {
        steps += 1;
        path.insert(next);
        next = *single_dir;
    }
    (next, steps)
}

fn collect_directions(
    trail: &[Vec<Tile>],
    cur: Node,
    end: Node,
    path: &BTreeSet<Node>,
) -> Vec<Node> {
    if cur == end {
        vec![]
    } else {
        [
            // Stepping up
            cur.1
                .checked_sub(1)
                .map(|y| (cur.0, y))
                .filter(|(x, y)| trail[*y][*x] != Tile::SlopeDown),
            // Stepping down
            Some((cur.0, cur.1 + 1)).filter(|(x, y)| trail[*y][*x] != Tile::SlopeUp),
            // Stepping left
            Some((cur.0 - 1, cur.1)).filter(|(x, y)| trail[*y][*x] != Tile::SlopeRight),
            // Stepping right
            Some((cur.0 + 1, cur.1)).filter(|(x, y)| trail[*y][*x] != Tile::SlopeLeft),
        ]
        .into_iter()
        .flatten()
        .filter(|(x, y)| trail[*y][*x] != Tile::Forest)
        .filter(|(x, y)| !path.contains(&(*x, *y)))
        .collect()
    }
}

fn walk_trail(
    graph: &BTreeMap<Node, BTreeSet<(Node, usize)>>,
    start: Node,
    end: Node,
    mut path: BTreeSet<Node>,
) -> Option<usize> {
    if start == end {
        Some(0)
    } else if path.insert(start) {
        graph.get(&start).and_then(|set| {
            set.iter()
                .flat_map(|(next, steps_to_next)| {
                    walk_trail(graph, *next, end, path.clone()).map(|s| steps_to_next + s)
                })
                .max()
        })
    } else {
        None
    }
}

fn process_trail(input: &str, slippery_slopes: bool) -> Result<Vec<Vec<Tile>>, String> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Ok(Tile::Path),
                    '#' => Ok(Tile::Forest),
                    '^' => Ok(if slippery_slopes {
                        Tile::SlopeUp
                    } else {
                        Tile::Path
                    }),
                    'v' => Ok(if slippery_slopes {
                        Tile::SlopeDown
                    } else {
                        Tile::Path
                    }),
                    '<' => Ok(if slippery_slopes {
                        Tile::SlopeLeft
                    } else {
                        Tile::Path
                    }),
                    '>' => Ok(if slippery_slopes {
                        Tile::SlopeRight
                    } else {
                        Tile::Path
                    }),
                    c => Err(format!("Not a valid tile. '{c}'")),
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_INPUT: &str = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn part1_test() {
        assert_eq!(longest_hike(PART1_INPUT, true), Ok(94));
    }

    #[test]
    fn part2_test() {
        assert_eq!(longest_hike(PART1_INPUT, false), Ok(154));
    }
}
