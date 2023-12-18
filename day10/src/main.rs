use std::{collections::VecDeque, io::Read};

#[derive(Debug, Clone, Copy)]
enum Loop {
    Shadowed,
    Outside,
    StraightPipeLeftOutside,
    StraightPipeRightOutside,
    StraightPipeUpOutside,
    StraightPipeDownOutside,
    NEPipeOuter,
    NEPipeInner,
    NWPipeOuter,
    NWPipeInner,
    SEPipeOuter,
    SEPipeInner,
    SWPipeOuter,
    SWPipeInner,
}

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day10_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = steps_to_farthest_point_in_loop(&input);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = enclosed_space_in_loop(&input);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn steps_to_farthest_point_in_loop(input: &str) -> Result<u64, String> {
    process_loop(input)?
        .into_iter()
        .filter(|dist| dist != &u64::MAX)
        .max()
        .ok_or("Failed to calculate max distance.".to_owned())
}

fn enclosed_space_in_loop(input: &str) -> Result<u64, String> {
    let lines = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    if let Some(start) = {
        lines
            .first()
            .and_then(|first| {
                first
                    .iter()
                    .position(|c| matches!(c, b'.' | b'|' | b'L' | b'J'))
                    .map(|x| (x, 0))
            })
            .or_else(|| {
                lines.last().and_then(|last| {
                    last.iter()
                        .position(|c| matches!(c, b'.' | b'|' | b'7' | b'F'))
                        .map(|x| (x, lines.len() - 1))
                })
            })
            .or_else(|| {
                lines.iter().enumerate().find_map(|(y, line)| {
                    line.first()
                        .filter(|first| matches!(first, b'.' | b'-' | b'7' | b'J'))
                        .map(|_| (0, y))
                        .or_else(|| {
                            line.last()
                                .filter(|first| matches!(first, b'.' | b'-' | b'7' | b'J'))
                                .map(|_| (line.len() - 1, y))
                        })
                })
            })
    } {
        let line_length = lines[0].len();
        let part_of_loop = process_loop(input)?;

        let mut map = vec![Loop::Shadowed; lines.len() * lines[0].len()];
        map[start.1 * line_length + start.0] = Loop::Outside;

        // mark all outside as Loop::Outside
        mark_all_outside(&start, line_length, &mut map, &lines, &part_of_loop)?;

        // Count all that is still shadowed and is ground
        Ok(map
            .iter()
            .filter(|tile| matches!(tile, Loop::Shadowed))
            .count() as u64)
    } else {
        Err("Could not find a starting point at the edge of map.".to_owned())
    }
}

fn process_loop(input: &str) -> Result<Vec<u64>, String> {
    let lines = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    if let Some(start) = lines
        .iter()
        .enumerate()
        .find_map(|(y, line)| line.iter().position(|c| c == &b'S').map(|x| (x, y)))
    {
        let line_length = lines[0].len();
        let mut distances = vec![u64::MAX; input.len()];
        distances[start.1 * line_length + start.0] = 0;

        let mut breadth = std::collections::VecDeque::new();
        breadth.push_back(start);

        while let Some(node) = breadth.pop_front() {
            breadth_update(&node, line_length, &lines, &mut distances, &mut breadth)?;
        }

        Ok(distances)
    } else {
        Err("Failed to find starting position.".to_owned())
    }
}

fn get_up<'a>(node: &(usize, usize), _line_length: usize, pipes: &'a [&[u8]]) -> Option<&'a u8> {
    if node.1 == 0 {
        None
    } else {
        Some(&pipes[node.1 - 1][node.0])
    }
}

fn get_down<'a>(node: &(usize, usize), line_length: usize, pipes: &'a [&[u8]]) -> Option<&'a u8> {
    if node.1 + 1 == line_length {
        None
    } else {
        Some(&pipes[node.1 + 1][node.0])
    }
}

fn get_right<'a>(node: &(usize, usize), line_length: usize, pipes: &'a [&[u8]]) -> Option<&'a u8> {
    if node.0 + 1 == line_length {
        None
    } else {
        Some(&pipes[node.1][node.0 + 1])
    }
}

fn get_left<'a>(node: &(usize, usize), _line_length: usize, pipes: &'a [&[u8]]) -> Option<&'a u8> {
    if node.0 == 0 {
        None
    } else {
        Some(&pipes[node.1][node.0 - 1])
    }
}

fn breadth_update(
    node: &(usize, usize),
    line_length: usize,
    pipes: &[&[u8]],
    distances: &mut [u64],
    search: &mut VecDeque<(usize, usize)>,
) -> Result<(), String> {
    let cur_dist = distances[node.1 * line_length + node.0];
    let dirs = match pipes[node.1][node.0] {
        b'S' => {
            let mut dirs = vec![];
            if matches!(get_up(node, line_length, pipes), Some(b'|' | b'7' | b'F')) {
                dirs.push('U');
            }
            if matches!(get_down(node, line_length, pipes), Some(b'|' | b'J' | b'L')) {
                dirs.push('D');
            }
            if matches!(get_left(node, line_length, pipes), Some(b'-' | b'L' | b'F')) {
                dirs.push('L');
            }
            if matches!(
                get_right(node, line_length, pipes),
                Some(b'-' | b'7' | b'J')
            ) {
                dirs.push('R');
            }
            dirs
        }
        b'-' => {
            vec!['R', 'L']
        }
        b'|' => {
            vec!['U', 'D']
        }
        b'J' => {
            vec!['U', 'L']
        }
        b'F' => {
            vec!['R', 'D']
        }
        b'7' => {
            vec!['L', 'D']
        }
        b'L' => {
            vec!['U', 'R']
        }
        b'.' => {
            vec![]
        }
        a => Err(format!("Unrecognized pipe. '{a}'"))?,
    };
    for dir in dirs {
        match dir {
            'U' => {
                if let Some(subbed) = node.1.checked_sub(1) {
                    if distances[subbed * line_length + node.0] == u64::MAX {
                        search.push_back((node.0, subbed));
                        distances[subbed * line_length + node.0] = cur_dist + 1;
                    }
                }
            }
            'D' => {
                if let Some(added) = node.1.checked_add(1) {
                    if added != pipes.len() && distances[added * line_length + node.0] == u64::MAX {
                        search.push_back((node.0, added));
                        distances[added * line_length + node.0] = cur_dist + 1;
                    }
                }
            }
            'L' => {
                if let Some(subbed) = node.0.checked_sub(1) {
                    if distances[node.1 * line_length + subbed] == u64::MAX {
                        search.push_back((subbed, node.1));
                        distances[node.1 * line_length + subbed] = cur_dist + 1;
                    }
                }
            }
            'R' => {
                if let Some(added) = node.0.checked_add(1) {
                    if added != line_length && distances[node.1 * line_length + added] == u64::MAX {
                        search.push_back((added, node.1));
                        distances[node.1 * line_length + added] = cur_dist + 1;
                    }
                }
            }
            _ => Err("Unrecognized direction.")?,
        }
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn mark_all_outside(
    start: &(usize, usize),
    line_length: usize,
    map: &mut [Loop],
    pipes: &[&[u8]],
    part_of_loop: &[u64],
) -> Result<(), String> {
    let mut breadth = VecDeque::from_iter([*start]);

    while let Some(node) = breadth.pop_front() {
        let cur_tile = pipes[node.1][node.0];
        let cur_tile_state = map[node.1 * line_length + node.0];

        #[allow(clippy::match_same_arms)]
        let dirs = match (cur_tile, cur_tile_state) {
            (b'S', _) => vec![],
            (b'.', Loop::Outside) => vec!['U', 'D', 'L', 'R'],
            (b'-', Loop::Outside) => vec!['U', 'D', 'L', 'R'],
            (b'-', Loop::StraightPipeUpOutside) => vec!['U', 'L', 'R'],
            (b'-', Loop::StraightPipeDownOutside) => vec!['D', 'L', 'R'],
            (b'|', Loop::Outside) => vec!['U', 'D', 'L', 'R'],
            (b'|', Loop::StraightPipeLeftOutside) => vec!['U', 'D', 'L'],
            (b'|', Loop::StraightPipeRightOutside) => vec!['U', 'D', 'R'],
            (b'L', Loop::Outside) => vec!['U', 'D', 'L', 'R'],
            (b'L', Loop::SWPipeOuter) => vec!['U', 'D', 'L', 'R'],
            (b'L', Loop::SWPipeInner) => vec!['U', 'R'],
            (b'F', Loop::Outside) => vec!['U', 'D', 'L', 'R'],
            (b'F', Loop::NWPipeOuter) => vec!['U', 'D', 'L', 'R'],
            (b'F', Loop::NWPipeInner) => vec!['D', 'R'],
            (b'7', Loop::Outside) => vec!['U', 'D', 'L', 'R'],
            (b'7', Loop::NEPipeOuter) => vec!['U', 'D', 'L', 'R'],
            (b'7', Loop::NEPipeInner) => vec!['D', 'L'],
            (b'J', Loop::Outside) => vec!['U', 'D', 'L', 'R'],
            (b'J', Loop::SEPipeOuter) => vec!['U', 'D', 'L', 'R'],
            (b'J', Loop::SEPipeInner) => vec!['U', 'L'],
            (a, b) => Err(format!(
                "Incorrect types. ({:?}, {b:?})",
                char::from_u32(u32::from(a))
            ))?,
        };

        for dir in dirs {
            #[allow(clippy::match_same_arms)]
            if match (dir, node) {
                ('U', (_, 0)) => true,
                ('D', (_, y)) if y + 1 == pipes.len() => true,
                ('L', (0, _)) => true,
                ('R', (x, _)) if x + 1 == line_length => true,
                _ => false,
            } {
                continue;
            }

            let (dir_node, dir_tile, dir_tile_state, is_in_loop) = match dir {
                'U' => (
                    (node.0, node.1 - 1),
                    pipes[node.1 - 1][node.0],
                    &mut map[(node.1 - 1) * line_length + node.0],
                    part_of_loop[(node.1 - 1) * line_length + node.0] != u64::MAX,
                ),
                'D' => (
                    (node.0, node.1 + 1),
                    pipes[node.1 + 1][node.0],
                    &mut map[(node.1 + 1) * line_length + node.0],
                    part_of_loop[(node.1 + 1) * line_length + node.0] != u64::MAX,
                ),
                'L' => (
                    (node.0 - 1, node.1),
                    pipes[node.1][node.0 - 1],
                    &mut map[node.1 * line_length + node.0 - 1],
                    part_of_loop[node.1 * line_length + node.0 - 1] != u64::MAX,
                ),
                'R' => (
                    (node.0 + 1, node.1),
                    pipes[node.1][node.0 + 1],
                    &mut map[node.1 * line_length + node.0 + 1],
                    part_of_loop[node.1 * line_length + node.0 + 1] != u64::MAX,
                ),
                _ => Err("Unknown direction.")?,
            };

            if !matches!(dir_tile_state, Loop::Shadowed) {
                continue;
            }

            let dir_tile = match dir_tile {
                b'S' => {
                    #[allow(clippy::match_same_arms)]
                    match (
                        (dir_node.1 != 0).then_some(()).and_then(|()| {
                            pipes
                                .get(dir_node.1 - 1)
                                .and_then(|line| line.get(dir_node.0))
                        }),
                        (dir_node.1 + 1 != pipes.len())
                            .then_some(())
                            .and_then(|()| {
                                pipes
                                    .get(dir_node.1 + 1)
                                    .and_then(|line| line.get(dir_node.0))
                            }),
                        pipes
                            .get(dir_node.1)
                            .filter(|_| dir_node.0 != 0)
                            .and_then(|line| line.get(dir_node.0 - 1)),
                        pipes
                            .get(dir_node.1)
                            .filter(|_| dir_node.0 + 1 != line_length)
                            .and_then(|line| line.get(dir_node.0 + 1)),
                    ) {
                        (Some(b'|' | b'7' | b'F'), Some(b'|' | b'L' | b'J'), _, _) => b'|',
                        (Some(b'|' | b'7' | b'F'), _, Some(b'-' | b'F' | b'L'), _) => b'J',
                        (Some(b'|' | b'7' | b'F'), _, _, Some(b'-' | b'7' | b'J')) => b'L',
                        (_, Some(b'|' | b'L' | b'J'), Some(b'-' | b'F' | b'L'), _) => b'7',
                        (_, Some(b'|' | b'L' | b'J'), _, Some(b'-' | b'7' | b'J')) => b'F',
                        (_, _, Some(b'-' | b'F' | b'L'), Some(b'-' | b'7' | b'J')) => b'-',
                        surroundings => Err(format!("Could not determinate S. {surroundings:?}"))?,
                    }
                }
                else_ => else_,
            };

            #[allow(clippy::match_same_arms)]
            match (dir, cur_tile, cur_tile_state, dir_tile, is_in_loop) {
                // Going to a tile that is not part of the loop always flags as Outside
                ('U' | 'D' | 'L' | 'R', _, _, _, false) => {
                    *dir_tile_state = Loop::Outside;
                }
                // From Outside to is_in_loop
                ('U', _, Loop::Outside, b'-', true) => {
                    *dir_tile_state = Loop::StraightPipeDownOutside;
                }
                ('D', _, Loop::Outside, b'-', true) => {
                    *dir_tile_state = Loop::StraightPipeUpOutside;
                }
                ('L', _, Loop::Outside, b'|', true) => {
                    *dir_tile_state = Loop::StraightPipeRightOutside;
                }
                ('R', _, Loop::Outside, b'|', true) => {
                    *dir_tile_state = Loop::StraightPipeLeftOutside;
                }
                ('D' | 'R', _, Loop::Outside, b'F', true) => {
                    *dir_tile_state = Loop::NWPipeOuter;
                }
                ('U' | 'R', _, Loop::Outside, b'L', true) => {
                    *dir_tile_state = Loop::SWPipeOuter;
                }
                ('U' | 'L', _, Loop::Outside, b'J', true) => {
                    *dir_tile_state = Loop::SEPipeOuter;
                }
                ('D' | 'L', _, Loop::Outside, b'7', true) => {
                    *dir_tile_state = Loop::NEPipeOuter;
                }
                // From '-' to '-'
                ('U', b'-', Loop::StraightPipeUpOutside, b'-', true) => {
                    *dir_tile_state = Loop::StraightPipeDownOutside;
                }
                ('L' | 'R', b'-', Loop::StraightPipeUpOutside, b'-', true) => {
                    *dir_tile_state = Loop::StraightPipeUpOutside;
                }
                ('D', b'-', Loop::StraightPipeDownOutside, b'-', true) => {
                    *dir_tile_state = Loop::StraightPipeUpOutside;
                }
                ('L' | 'R', b'-', Loop::StraightPipeDownOutside, b'-', true) => {
                    *dir_tile_state = Loop::StraightPipeDownOutside;
                }
                // From '-' to '7'
                ('R', b'-', Loop::StraightPipeUpOutside, b'7', true) => {
                    *dir_tile_state = Loop::NEPipeOuter;
                }
                ('D', b'-', Loop::StraightPipeDownOutside, b'7', true) => {
                    *dir_tile_state = Loop::NEPipeOuter;
                }
                ('R', b'-', Loop::StraightPipeDownOutside, b'7', true) => {
                    *dir_tile_state = Loop::NEPipeInner;
                }
                // From '-' to 'J'
                ('U', b'-', Loop::StraightPipeUpOutside, b'J', true) => {
                    *dir_tile_state = Loop::SEPipeOuter;
                }
                ('R', b'-', Loop::StraightPipeUpOutside, b'J', true) => {
                    *dir_tile_state = Loop::SEPipeInner;
                }
                ('R', b'-', Loop::StraightPipeDownOutside, b'J', true) => {
                    *dir_tile_state = Loop::SEPipeOuter;
                }
                // From '-' to 'F'
                ('L', b'-', Loop::StraightPipeUpOutside, b'F', true) => {
                    *dir_tile_state = Loop::NWPipeOuter;
                }
                ('D', b'-', Loop::StraightPipeDownOutside, b'F', true) => {
                    *dir_tile_state = Loop::NWPipeOuter;
                }
                ('L', b'-', Loop::StraightPipeDownOutside, b'F', true) => {
                    *dir_tile_state = Loop::NWPipeInner;
                }
                // From '-' to 'L'
                ('U', b'-', Loop::StraightPipeUpOutside, b'L', true) => {
                    *dir_tile_state = Loop::SWPipeOuter;
                }
                ('L', b'-', Loop::StraightPipeUpOutside, b'L', true) => {
                    *dir_tile_state = Loop::SWPipeInner;
                }
                ('L', b'-', Loop::StraightPipeDownOutside, b'L', true) => {
                    *dir_tile_state = Loop::SWPipeOuter;
                }
                // From '|' to '|'
                ('U' | 'D', b'|', Loop::StraightPipeLeftOutside, b'|', true) => {
                    *dir_tile_state = Loop::StraightPipeLeftOutside;
                }
                ('L', b'|', Loop::StraightPipeLeftOutside, b'|', true) => {
                    *dir_tile_state = Loop::StraightPipeRightOutside;
                }
                ('U' | 'D', b'|', Loop::StraightPipeRightOutside, b'|', true) => {
                    *dir_tile_state = Loop::StraightPipeRightOutside;
                }
                ('R', b'|', Loop::StraightPipeRightOutside, b'|', true) => {
                    *dir_tile_state = Loop::StraightPipeLeftOutside;
                }
                // From '|' to '7'
                ('U', b'|', Loop::StraightPipeLeftOutside, b'7', true) => {
                    *dir_tile_state = Loop::NEPipeInner;
                }
                ('L', b'|', Loop::StraightPipeLeftOutside, b'7', true) => {
                    *dir_tile_state = Loop::NEPipeOuter;
                }
                ('U', b'|', Loop::StraightPipeRightOutside, b'7', true) => {
                    *dir_tile_state = Loop::NEPipeOuter;
                }
                // From '|' to 'J'
                ('D', b'|', Loop::StraightPipeLeftOutside, b'J', true) => {
                    *dir_tile_state = Loop::SEPipeInner;
                }
                ('L', b'|', Loop::StraightPipeLeftOutside, b'J', true) => {
                    *dir_tile_state = Loop::SEPipeOuter;
                }
                ('D', b'|', Loop::StraightPipeRightOutside, b'J', true) => {
                    *dir_tile_state = Loop::SEPipeOuter;
                }
                // From '|' to 'F'
                ('U', b'|', Loop::StraightPipeLeftOutside, b'F', true) => {
                    *dir_tile_state = Loop::NWPipeOuter;
                }
                ('U', b'|', Loop::StraightPipeRightOutside, b'F', true) => {
                    *dir_tile_state = Loop::NWPipeInner;
                }
                ('R', b'|', Loop::StraightPipeRightOutside, b'F', true) => {
                    *dir_tile_state = Loop::NWPipeOuter;
                }
                // From '|' to 'L'
                ('D', b'|', Loop::StraightPipeLeftOutside, b'L', true) => {
                    *dir_tile_state = Loop::SWPipeOuter;
                }
                ('D', b'|', Loop::StraightPipeRightOutside, b'L', true) => {
                    *dir_tile_state = Loop::SWPipeInner;
                }
                ('R', b'|', Loop::StraightPipeRightOutside, b'L', true) => {
                    *dir_tile_state = Loop::SWPipeOuter;
                }
                // From 'L' to '-'
                ('R', b'L', Loop::SWPipeOuter, b'-', true) => {
                    *dir_tile_state = Loop::StraightPipeDownOutside;
                }
                ('D', b'L', Loop::SWPipeOuter, b'-', true) => {
                    *dir_tile_state = Loop::StraightPipeUpOutside;
                }
                ('R', b'L', Loop::SWPipeInner, b'-', true) => {
                    *dir_tile_state = Loop::StraightPipeUpOutside;
                }
                // From 'L' to '|'
                ('U', b'L', Loop::SWPipeOuter, b'|', true) => {
                    *dir_tile_state = Loop::StraightPipeLeftOutside;
                }
                ('L', b'L', Loop::SWPipeOuter, b'|', true) => {
                    *dir_tile_state = Loop::StraightPipeRightOutside;
                }
                ('U', b'L', Loop::SWPipeInner, b'|', true) => {
                    *dir_tile_state = Loop::StraightPipeRightOutside;
                }
                // From 'L' to 'J'
                ('L' | 'R', b'L', Loop::SWPipeOuter, b'J', true) => {
                    *dir_tile_state = Loop::SEPipeOuter;
                }
                ('R', b'L', Loop::SWPipeInner, b'J', true) => {
                    *dir_tile_state = Loop::SEPipeInner;
                }
                // From 'L' to 'F'
                ('U' | 'D', b'L', Loop::SWPipeOuter, b'F', true) => {
                    *dir_tile_state = Loop::NWPipeOuter;
                }
                ('U', b'L', Loop::SWPipeInner, b'F', true) => {
                    *dir_tile_state = Loop::NWPipeInner;
                }
                // From 'L' to '7'
                ('U' | 'R', b'L', Loop::SWPipeOuter, b'7', true) => {
                    *dir_tile_state = Loop::NEPipeInner;
                }
                ('D' | 'L', b'L', Loop::SWPipeOuter, b'7', true) => {
                    *dir_tile_state = Loop::NEPipeOuter;
                }
                ('U' | 'R', b'L', Loop::SWPipeInner, b'7', true) => {
                    *dir_tile_state = Loop::NEPipeOuter;
                }
                ('D' | 'L', b'L', Loop::SWPipeInner, b'7', true) => {
                    *dir_tile_state = Loop::NEPipeInner;
                }
                // From 'F' to '-'
                ('R', b'F', Loop::NWPipeOuter, b'-', true) => {
                    *dir_tile_state = Loop::StraightPipeUpOutside;
                }
                ('U', b'F', Loop::NWPipeOuter, b'-', true) => {
                    *dir_tile_state = Loop::StraightPipeDownOutside;
                }
                ('R', b'F', Loop::NWPipeInner, b'-', true) => {
                    *dir_tile_state = Loop::StraightPipeDownOutside;
                }
                // From 'F' to '|'
                ('D', b'F', Loop::NWPipeOuter, b'|', true) => {
                    *dir_tile_state = Loop::StraightPipeLeftOutside;
                }
                ('L', b'F', Loop::NWPipeOuter, b'|', true) => {
                    *dir_tile_state = Loop::StraightPipeRightOutside;
                }
                ('D', b'F', Loop::NWPipeInner, b'|', true) => {
                    *dir_tile_state = Loop::StraightPipeRightOutside;
                }
                // From 'F' to 'J'
                ('U' | 'L', b'F', Loop::NWPipeOuter, b'J', true) => {
                    *dir_tile_state = Loop::SEPipeOuter;
                }
                ('D' | 'R', b'F', Loop::NWPipeOuter, b'J', true) => {
                    *dir_tile_state = Loop::SEPipeInner;
                }
                ('U' | 'L', b'F', Loop::NWPipeInner, b'J', true) => {
                    *dir_tile_state = Loop::SEPipeInner;
                }
                ('D' | 'R', b'F', Loop::NWPipeInner, b'J', true) => {
                    *dir_tile_state = Loop::SEPipeOuter;
                }
                // From 'F' to 'L'
                ('U' | 'D', b'F', Loop::NWPipeOuter, b'L', true) => {
                    *dir_tile_state = Loop::SWPipeOuter;
                }
                ('D', b'F', Loop::NWPipeInner, b'L', true) => {
                    *dir_tile_state = Loop::SWPipeInner;
                }
                // From 'F' to '7'
                ('L' | 'R', b'F', Loop::NWPipeOuter, b'7', true) => {
                    *dir_tile_state = Loop::NEPipeOuter;
                }
                ('L' | 'R', b'F', Loop::NWPipeInner, b'7', true) => {
                    *dir_tile_state = Loop::NEPipeInner;
                }
                // From '7' to '-'
                ('U', b'7', Loop::NEPipeOuter, b'-', true) => {
                    *dir_tile_state = Loop::StraightPipeDownOutside;
                }
                ('L', b'7', Loop::NEPipeOuter, b'-', true) => {
                    *dir_tile_state = Loop::StraightPipeUpOutside;
                }
                ('L', b'7', Loop::NEPipeInner, b'-', true) => {
                    *dir_tile_state = Loop::StraightPipeDownOutside;
                }
                // From '7' to '|'
                ('D', b'7', Loop::NEPipeOuter, b'|', true) => {
                    *dir_tile_state = Loop::StraightPipeRightOutside;
                }
                ('R', b'7', Loop::NEPipeOuter, b'|', true) => {
                    *dir_tile_state = Loop::StraightPipeLeftOutside;
                }
                ('D', b'7', Loop::NEPipeInner, b'|', true) => {
                    *dir_tile_state = Loop::StraightPipeLeftOutside;
                }
                // From '7' to 'J'
                ('U' | 'D', b'7', Loop::NEPipeOuter, b'J', true) => {
                    *dir_tile_state = Loop::SEPipeOuter;
                }
                ('U' | 'D', b'7', Loop::NEPipeInner, b'J', true) => {
                    *dir_tile_state = Loop::SEPipeInner;
                }
                // From '7' to 'L'
                ('U' | 'R', b'7', Loop::NEPipeOuter, b'L', true) => {
                    *dir_tile_state = Loop::SWPipeOuter;
                }
                ('D' | 'L', b'7', Loop::NEPipeOuter, b'L', true) => {
                    *dir_tile_state = Loop::SWPipeInner;
                }
                ('U' | 'R', b'7', Loop::NEPipeInner, b'L', true) => {
                    *dir_tile_state = Loop::SWPipeInner;
                }
                ('D' | 'L', b'7', Loop::NEPipeInner, b'L', true) => {
                    *dir_tile_state = Loop::SWPipeOuter;
                }
                // From '7' to 'F'
                ('L' | 'R', b'7', Loop::NEPipeOuter, b'F', true) => {
                    *dir_tile_state = Loop::NWPipeOuter;
                }
                ('L' | 'R', b'7', Loop::NEPipeInner, b'F', true) => {
                    *dir_tile_state = Loop::NWPipeInner;
                }
                // From 'J' to '-'
                ('D', b'J', Loop::SEPipeOuter, b'-', true) => {
                    *dir_tile_state = Loop::StraightPipeUpOutside;
                }
                ('L', b'J', Loop::SEPipeOuter, b'-', true) => {
                    *dir_tile_state = Loop::StraightPipeDownOutside;
                }
                ('L', b'J', Loop::SEPipeInner, b'-', true) => {
                    *dir_tile_state = Loop::StraightPipeUpOutside;
                }
                // From 'J' to '|'
                ('U', b'J', Loop::SEPipeOuter, b'|', true) => {
                    *dir_tile_state = Loop::StraightPipeRightOutside;
                }
                ('R', b'J', Loop::SEPipeOuter, b'|', true) => {
                    *dir_tile_state = Loop::StraightPipeLeftOutside;
                }
                ('U', b'J', Loop::SEPipeInner, b'|', true) => {
                    *dir_tile_state = Loop::StraightPipeLeftOutside;
                }
                // From 'J' to '7'
                ('U' | 'D', b'J', Loop::SEPipeOuter, b'7', true) => {
                    *dir_tile_state = Loop::NEPipeOuter;
                }
                ('U' | 'D', b'J', Loop::SEPipeInner, b'7', true) => {
                    *dir_tile_state = Loop::NEPipeInner;
                }
                // From 'J' to 'F'
                ('U' | 'L', b'J', Loop::SEPipeOuter, b'F', true) => {
                    *dir_tile_state = Loop::NWPipeInner;
                }
                ('D' | 'R', b'J', Loop::SEPipeOuter, b'F', true) => {
                    *dir_tile_state = Loop::NWPipeOuter;
                }
                ('U' | 'L', b'J', Loop::SEPipeInner, b'F', true) => {
                    *dir_tile_state = Loop::NWPipeOuter;
                }
                ('D' | 'R', b'J', Loop::SEPipeInner, b'F', true) => {
                    *dir_tile_state = Loop::NWPipeInner;
                }
                // From 'J' to 'L'
                ('L' | 'R', b'J', Loop::SEPipeOuter, b'L', true) => {
                    *dir_tile_state = Loop::SWPipeOuter;
                }
                ('L' | 'R', b'J', Loop::SEPipeInner, b'L', true) => {
                    *dir_tile_state = Loop::SWPipeInner;
                }
                a => Err(format!("Unknown combination {a:?}."))?,
            };

            // If this is reached, the tile one the direction was updated
            match dir {
                'U' => breadth.push_back((node.0, node.1 - 1)),
                'D' => breadth.push_back((node.0, node.1 + 1)),
                'L' => breadth.push_back((node.0 - 1, node.1)),
                'R' => breadth.push_back((node.0 + 1, node.1)),
                _ => Err("Unknown direction.")?,
            };
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_INPUT1: &str = r".....
.S-7.
.|.|.
.L-J.
.....";

    const PART1_INPUT2: &str = r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    const PART2_INPUT1: &str = r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const PART2_INPUT2: &str = r"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    const PART2_INPUT3: &str = r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const PART2_INPUT4: &str = r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn part1_test() {
        assert_eq!(steps_to_farthest_point_in_loop(PART1_INPUT1), Ok(4));
        assert_eq!(steps_to_farthest_point_in_loop(PART1_INPUT2), Ok(8));
    }

    #[test]
    fn part2_test() {
        assert_eq!(enclosed_space_in_loop(PART2_INPUT1), Ok(4));
        assert_eq!(enclosed_space_in_loop(PART2_INPUT2), Ok(4));
        assert_eq!(enclosed_space_in_loop(PART2_INPUT3), Ok(8));
        assert_eq!(enclosed_space_in_loop(PART2_INPUT4), Ok(10));
    }
}
