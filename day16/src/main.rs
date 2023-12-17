use std::{collections::VecDeque, io::Read};

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day16_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = energized_tiles(&input);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = maximize_energized_tiles(&input);
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
enum RayDirection {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
enum ObstacleInteraction {
    Deflaction(RayDirection),
    Split(RayDirection, RayDirection),
    Wall,
}

fn energized_tiles(input: &str) -> Result<u64, String> {
    let tiles = input.lines().map(str::as_bytes).collect::<Vec<_>>();

    run_energization_for_starting_tile(&tiles, (0, 0, RayDirection::East))
}

fn maximize_energized_tiles(input: &str) -> Result<u64, String> {
    let tiles = input.lines().map(str::as_bytes).collect::<Vec<_>>();

    let line_count = tiles.len();
    let line_length = tiles[0].len();

    (0..line_length)
        .map(|x| (x, 0, RayDirection::South))
        .chain((0..line_length).map(|x| (x, line_count - 1, RayDirection::North)))
        .chain((0..line_count).map(|y| (0, y, RayDirection::East)))
        .chain((0..line_count).map(|y| (line_length - 1, y, RayDirection::West)))
        .map(|start_ray| run_energization_for_starting_tile(&tiles, start_ray))
        .max()
        .ok_or("Could not find count of energized tiles.")?
}

fn run_energization_for_starting_tile(
    tiles: &[&[u8]],
    start_ray: (usize, usize, RayDirection),
) -> Result<u64, String> {
    let mut energized = vec![vec![false; tiles[0].len()]; tiles.len()];

    let line_count = tiles.len();
    let line_length = tiles[0].len();

    let start_rays = get_next_direction(
        (start_ray.0, start_ray.1),
        start_ray.2,
        tiles[start_ray.1][start_ray.0],
        (line_length, line_count),
    )
    .and_then(|obst_inter| match obst_inter {
        ObstacleInteraction::Deflaction(dir) => Ok(vec![(start_ray.0, start_ray.1, dir)]),
        ObstacleInteraction::Split(split_a, split_b) => Ok(vec![
            (start_ray.0, start_ray.1, split_a),
            (start_ray.0, start_ray.1, split_b),
        ]),
        ObstacleInteraction::Wall => Err("Ray started point to wall.".to_owned()),
    })
    .unwrap_or(vec![start_ray]);

    let mut rays_to_process = VecDeque::from_iter(start_rays);
    let mut rays_cache = vec![];

    while let Some((ray_x, ray_y, ray_dir)) = rays_to_process.pop_front() {
        rays_cache.push((ray_x, ray_y, ray_dir));
        let stop = match ray_dir {
            RayDirection::North => next_obstacle_north((ray_x, ray_y), tiles),
            RayDirection::South => next_obstacle_south((ray_x, ray_y), tiles),
            RayDirection::East => next_obstacle_east((ray_x, ray_y), tiles),
            RayDirection::West => next_obstacle_west((ray_x, ray_y), tiles),
        };
        let obstacle = tiles[stop.1][stop.0];

        let next_dir = get_next_direction(stop, ray_dir, obstacle, (line_length, line_count))?;

        match next_dir {
            ObstacleInteraction::Deflaction(def) => {
                add_new_ray_to_queue((stop.0, stop.1, def), &rays_cache, &mut rays_to_process);
            }
            ObstacleInteraction::Split(split_a, split_b) => {
                add_new_ray_to_queue((stop.0, stop.1, split_a), &rays_cache, &mut rays_to_process);
                add_new_ray_to_queue((stop.0, stop.1, split_b), &rays_cache, &mut rays_to_process);
            }
            ObstacleInteraction::Wall => (),
        };

        energize_grid(&mut energized, (ray_x, ray_y), stop)?;
    }

    energized
        .iter()
        .map(|tile| u64::try_from(tile.iter().filter(|energy| **energy).count()))
        .sum::<Result<u64, std::num::TryFromIntError>>()
        .map_err(|err| format!("Failed to calculate energized tiles count. '{err}'"))
}

fn add_new_ray_to_queue(
    new_ray: (usize, usize, RayDirection),
    ray_cache: &[(usize, usize, RayDirection)],
    rays_to_process: &mut VecDeque<(usize, usize, RayDirection)>,
) {
    if !ray_cache.contains(&new_ray) {
        rays_to_process.push_back(new_ray);
    }
}

fn get_next_direction(
    stop: (usize, usize),
    ray_dir: RayDirection,
    obstacle: u8,
    grid_size: (usize, usize),
) -> Result<ObstacleInteraction, String> {
    match (obstacle, ray_dir) {
        (b'|', RayDirection::East | RayDirection::West) => Ok(ObstacleInteraction::Split(
            RayDirection::North,
            RayDirection::South,
        )),
        (b'-', RayDirection::North | RayDirection::South) => Ok(ObstacleInteraction::Split(
            RayDirection::East,
            RayDirection::West,
        )),
        (b'|' | b'-', dir) => Ok(ObstacleInteraction::Deflaction(dir)),
        (b'/', RayDirection::North) | (b'\\', RayDirection::South) => {
            Ok(ObstacleInteraction::Deflaction(RayDirection::East))
        }
        (b'/', RayDirection::South) | (b'\\', RayDirection::North) => {
            Ok(ObstacleInteraction::Deflaction(RayDirection::West))
        }
        (b'/', RayDirection::East) | (b'\\', RayDirection::West) => {
            Ok(ObstacleInteraction::Deflaction(RayDirection::North))
        }
        (b'/', RayDirection::West) | (b'\\', RayDirection::East) => {
            Ok(ObstacleInteraction::Deflaction(RayDirection::South))
        }
        (b'.', RayDirection::North) => {
            if stop.1 == 0 {
                Ok(ObstacleInteraction::Wall)
            } else {
                Err(format!(
                    "Ray traveling north stopped unexpectedly. '{stop:?}'",
                ))
            }
        }
        (b'.', RayDirection::South) => {
            if stop.1 == grid_size.1 - 1 {
                Ok(ObstacleInteraction::Wall)
            } else {
                Err(format!(
                    "Ray traveling south stopped unexpectedly. '{stop:?}'",
                ))
            }
        }
        (b'.', RayDirection::East) => {
            if stop.0 == grid_size.0 - 1 {
                Ok(ObstacleInteraction::Wall)
            } else {
                Err(format!(
                    "Ray traveling east stopped unexpectedly. '{stop:?}'",
                ))
            }
        }
        (b'.', RayDirection::West) => {
            if stop.0 == 0 {
                Ok(ObstacleInteraction::Wall)
            } else {
                Err(format!(
                    "Ray traveling west stopped unexpectedly. '{stop:?}'",
                ))
            }
        }
        _ => Err(format!(
            "Unknown mirror orientation. '{:?}'",
            char::from_u32(u32::from(obstacle))
        )),
    }
}

fn energize_grid(
    energized: &mut [Vec<bool>],
    begin: (usize, usize),
    end: (usize, usize),
) -> Result<(), String> {
    match (begin, end) {
        ((bx, by), (ex, ey)) if bx == ex => {
            energized[by.min(ey)..=by.max(ey)]
                .iter_mut()
                .for_each(|tile| tile[bx] = true);
        }
        ((bx, by), (ex, ey)) if by == ey => {
            energized[by][bx.min(ex)..=bx.max(ex)]
                .iter_mut()
                .for_each(|tile| *tile = true);
        }
        _ => Err(format!("Can't process diagonal ray. ({begin:?}, {end:?})"))?,
    }

    Ok(())
}

fn next_obstacle_north(ray_position: (usize, usize), lines: &[&[u8]]) -> (usize, usize) {
    lines
        .iter()
        .enumerate()
        .rev()
        .find(|(y, line)| y < &ray_position.1 && line[ray_position.0] != b'.')
        .map_or((ray_position.0, 0), |(obstacle_ypos, _)| {
            (ray_position.0, obstacle_ypos)
        })
}

fn next_obstacle_south(ray_position: (usize, usize), lines: &[&[u8]]) -> (usize, usize) {
    let line_count = lines.len();
    lines
        .iter()
        .enumerate()
        .position(|(y, line)| y > ray_position.1 && line[ray_position.0] != b'.')
        .map_or((ray_position.0, line_count - 1), |obstacle_ypos| {
            (ray_position.0, obstacle_ypos)
        })
}

fn next_obstacle_east(ray_position: (usize, usize), lines: &[&[u8]]) -> (usize, usize) {
    let line_len = lines[ray_position.1].len();
    lines[ray_position.1]
        .iter()
        .enumerate()
        .position(|(x, tile)| x > ray_position.0 && tile != &b'.')
        .map_or((line_len - 1, ray_position.1), |obstacle_xpos| {
            (obstacle_xpos, ray_position.1)
        })
}

fn next_obstacle_west(ray_position: (usize, usize), lines: &[&[u8]]) -> (usize, usize) {
    lines[ray_position.1]
        .iter()
        .enumerate()
        .rev()
        .find(|(x, tile)| x < &ray_position.0 && **tile != b'.')
        .map_or((0, ray_position.1), |(obstacle_xpos, _)| {
            (obstacle_xpos, ray_position.1)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn part1_test() {
        assert_eq!(energized_tiles(PART1_INPUT), Ok(46));
    }

    #[test]
    fn part2_test() {
        assert_eq!(maximize_energized_tiles(PART1_INPUT), Ok(51));
    }
}
