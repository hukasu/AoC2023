use std::{collections::VecDeque, io::Read};

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day17_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = path_of_least_heat_loss(&input, false);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = path_of_least_heat_loss(&input, true);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CrucibleDirection {
    North,
    South,
    East,
    West,
}

impl From<CrucibleDirection> for usize {
    fn from(value: CrucibleDirection) -> Self {
        match value {
            CrucibleDirection::North => 0,
            CrucibleDirection::South => 1,
            CrucibleDirection::East => 2,
            CrucibleDirection::West => 3,
        }
    }
}

fn path_of_least_heat_loss(input: &str, using_mega_crucible: bool) -> Result<u64, String> {
    let heat_loss_map = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|c| u64::try_from(c - b'0'))
                .collect::<Result<Vec<u64>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.to_string())?;

    let y_max = heat_loss_map.len();
    let x_max = heat_loss_map[0].len();

    let mut cum_heat_loss_map = vec![vec![vec![u64::MAX / 2; x_max]; y_max]; 4];
    cum_heat_loss_map[0][0][0] = 0;
    cum_heat_loss_map[1][0][0] = 0;
    cum_heat_loss_map[2][0][0] = 0;
    cum_heat_loss_map[3][0][0] = 0;

    let mut path_testing = VecDeque::from_iter([
        (0, 0, CrucibleDirection::East),
        (0, 0, CrucibleDirection::South),
    ]);

    while let Some((x, y, dir)) = path_testing.pop_front() {
        let start_pos_heat_lost = cum_heat_loss_map[usize::from(dir)][y][x];
        let reacheable = if using_mega_crucible {
            get_reacheable_tiles(x, y, dir, x_max, y_max, 4, 10)
        } else {
            get_reacheable_tiles(x, y, dir, x_max, y_max, 1, 3)
        };
        let turns = get_possible_turns(dir);

        for (next_x, next_y) in reacheable {
            let move_heat_loss = get_move_heat_loss(x, y, dir, next_x, next_y, &heat_loss_map)?;
            for turn in turns {
                if cum_heat_loss_map[usize::from(turn)][next_y][next_x]
                    > start_pos_heat_lost + move_heat_loss
                {
                    cum_heat_loss_map[usize::from(turn)][next_y][next_x] =
                        start_pos_heat_lost + move_heat_loss;
                    path_testing.push_back((next_x, next_y, turn));
                }
            }
        }
    }

    // cum_heat_loss_map.iter().for_each(|grid| {
    //     grid.iter().for_each(|line| println!("{line:?}"));
    //     println!();
    // });

    cum_heat_loss_map
        .iter()
        .filter_map(|end| end.last().and_then(|last_row| last_row.last()).copied())
        .min()
        .ok_or("Failed to calculate heat loss.".to_owned())
}

fn get_reacheable_tiles(
    x: usize,
    y: usize,
    dir: CrucibleDirection,
    x_max: usize,
    y_max: usize,
    move_min: usize,
    move_max: usize,
) -> Vec<(usize, usize)> {
    match dir {
        CrucibleDirection::North => (move_min..=move_max)
            .filter_map(|i| if y >= i { Some((x, y - i)) } else { None })
            .collect(),
        CrucibleDirection::South => (move_min..=move_max)
            .filter_map(|i| {
                if y + i < y_max {
                    Some((x, y + i))
                } else {
                    None
                }
            })
            .collect(),
        CrucibleDirection::East => (move_min..=move_max)
            .filter_map(|i| {
                if x + i < x_max {
                    Some((x + i, y))
                } else {
                    None
                }
            })
            .collect(),
        CrucibleDirection::West => (move_min..=move_max)
            .filter_map(|i| if x >= i { Some((x - i, y)) } else { None })
            .collect(),
    }
}

fn get_possible_turns(dir: CrucibleDirection) -> [CrucibleDirection; 2] {
    match dir {
        CrucibleDirection::North | CrucibleDirection::South => {
            [CrucibleDirection::East, CrucibleDirection::West]
        }
        CrucibleDirection::East | CrucibleDirection::West => {
            [CrucibleDirection::North, CrucibleDirection::South]
        }
    }
}

fn get_move_heat_loss(
    x: usize,
    y: usize,
    dir: CrucibleDirection,
    next_x: usize,
    next_y: usize,
    heat_loss_map: &[Vec<u64>],
) -> Result<u64, String> {
    match (x, y, dir, next_x, next_y) {
        (x, y, CrucibleDirection::North, next_x, next_y) if x == next_x && y > next_y => {
            Ok(heat_loss_map[next_y..y].iter().map(|range| range[x]).sum())
        }
        (x, y, CrucibleDirection::South, next_x, next_y) if x == next_x && y < next_y => {
            Ok(heat_loss_map[(y + 1)..=next_y]
                .iter()
                .map(|range| range[x])
                .sum())
        }
        (x, y, CrucibleDirection::East, next_x, next_y) if x < next_x && y == next_y => {
            Ok(heat_loss_map[y][(x + 1)..=next_x].iter().sum())
        }
        (x, y, CrucibleDirection::West, next_x, next_y) if x > next_x && y == next_y => {
            Ok(heat_loss_map[y][next_x..x].iter().sum())
        }
        err => Err(format!(
            "Invalid combination while calculating move heat loss. '{err:?}'"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_INPUT: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    #[test]
    fn part1_test() {
        assert_eq!(path_of_least_heat_loss(PART1_INPUT, false), Ok(102));
    }

    #[test]
    fn part2_test() {
        assert_eq!(path_of_least_heat_loss(PART1_INPUT, true), Ok(94));
    }
}
