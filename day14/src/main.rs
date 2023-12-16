use std::io::Read;

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day14_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = calculate_load_after_roll(&input);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = calculate_load_after_roll_cycle(&input);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn calculate_load_after_roll(input: &str) -> Result<u64, String> {
    let (round_rocks, cube_rocks) = get_rock_locations(input);
    let rolled_rocks = roll_rocks_north(&round_rocks, &cube_rocks);
    let rows = input.lines().count();
    rolled_rocks
        .iter()
        .map(|(_rock_x, rock_y)| u64::try_from(rows - rock_y).map_err(|err| err.to_string()))
        .sum()
}

fn calculate_load_after_roll_cycle(input: &str) -> Result<u64, String> {
    const CYCLES: usize = 1_000_000_000;
    let rows = input.lines().count();
    let (round_rocks, cube_rocks) = get_rock_locations(input);

    let mut cache = Vec::new();
    let mut cycled = round_rocks;
    for i in 0..CYCLES {
        if i % 1000 == 999 {
            println!("{i}");
        }
        let new = roll_cycle(&cycled, &cube_rocks);
        cycled = new.clone();
        if cache.contains(&new) {
            break;
        }
        cache.push(new);
    }

    if let Some(cycle_start) = cache
        .iter()
        .position(|cached_cycle| cached_cycle.eq(&cycled))
    {
        let cycle_length = cache.len() - cycle_start;
        let cycle_offset = (CYCLES - cycle_start) % cycle_length;
        let arrangement_at_end = cycle_start + cycle_offset - 1;

        cache[arrangement_at_end]
            .iter()
            .map(|(_rock_x, rock_y)| u64::try_from(rows - rock_y).map_err(|err| err.to_string()))
            .sum()
    } else {
        Err("Failed to find cycle.".to_owned())
    }
}

fn roll_cycle(
    round_rocks: &[(usize, usize)],
    cube_rocks: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    let north = roll_rocks_north(round_rocks, cube_rocks);
    let west = roll_rocks_west(&north, cube_rocks);
    let south = roll_rocks_south(&west, cube_rocks);
    let mut east = roll_rocks_east(&south, cube_rocks);
    east.sort_unstable();
    east
}

fn roll_rocks_north(
    round_rocks: &[(usize, usize)],
    cube_rocks: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    let mut rolled = vec![];
    for (rock_x, rock_y) in round_rocks {
        let closest_block = cube_rocks
            .iter()
            .rev()
            // Rocks that are on the same column
            .filter(|(cube_x, _cube_y)| cube_x == rock_x)
            // First blockage to the north of current rock
            .find(|(_cube_x, cube_y)| cube_y < rock_y)
            .copied()
            .map_or((*rock_x, 0), |(block_x, block_y)| (block_x, block_y + 1));
        let rocks_between_rock_and_block = round_rocks
            .iter()
            // Rocks that are on the same column
            .filter(|(round_x, _round_y)| round_x == rock_x)
            // Rocks that are farther than the blockage
            .filter(|(_round_x, round_y)| round_y >= &closest_block.1)
            // Rocks between blockage and current rock
            .filter(|(_round_x, round_y)| round_y < rock_y)
            .count();
        rolled.push((*rock_x, closest_block.1 + rocks_between_rock_and_block));
    }
    rolled
}

fn roll_rocks_south(
    round_rocks: &[(usize, usize)],
    cube_rocks: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    if let Some(south_edge) = cube_rocks.iter().map(|(_, y)| y).max() {
        let mut rolled = vec![];
        for (rock_x, rock_y) in round_rocks {
            let closest_block = cube_rocks
                .iter()
                // Rocks that are on the same column
                .filter(|(cube_x, _cube_y)| cube_x == rock_x)
                // First blockage to the south of current rock
                .find(|(_cube_x, cube_y)| cube_y > rock_y)
                .copied()
                .map_or((*rock_x, *south_edge), |(block_x, block_y)| {
                    (block_x, block_y - 1)
                });
            let rocks_between_rock_and_block = round_rocks
                .iter()
                // Rocks that are on the same column
                .filter(|(round_x, _round_y)| round_x == rock_x)
                // Rocks that are farther than the blockage
                .filter(|(_round_x, round_y)| round_y <= &closest_block.1)
                // Rocks between blockage and current rock
                .filter(|(_round_x, round_y)| round_y > rock_y)
                .count();
            rolled.push((*rock_x, closest_block.1 - rocks_between_rock_and_block));
        }
        rolled
    } else {
        vec![]
    }
}

fn roll_rocks_west(
    round_rocks: &[(usize, usize)],
    cube_rocks: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    let mut rolled = vec![];
    for (rock_x, rock_y) in round_rocks {
        let closest_block = cube_rocks
            .iter()
            .rev()
            // Rocks that are on the same row
            .filter(|(_cube_x, cube_y)| cube_y == rock_y)
            // First blockage to the west of current rock
            .find(|(cube_x, _cube_y)| cube_x < rock_x)
            .copied()
            .map_or((0, *rock_y), |(block_x, block_y)| (block_x + 1, block_y));
        let rocks_between_rock_and_block = round_rocks
            .iter()
            // Rocks that are on the same row
            .filter(|(_round_x, round_y)| round_y == rock_y)
            // Rocks that are farther than the blockage
            .filter(|(round_x, _round_y)| round_x >= &closest_block.0)
            // Rocks between blockage and current rock
            .filter(|(round_x, _round_y)| round_x < rock_x)
            .count();
        rolled.push((closest_block.0 + rocks_between_rock_and_block, *rock_y));
    }
    rolled
}

fn roll_rocks_east(
    round_rocks: &[(usize, usize)],
    cube_rocks: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    if let Some(east_edge) = cube_rocks.iter().map(|(x, _)| x).max() {
        let mut rolled = vec![];
        for (rock_x, rock_y) in round_rocks {
            let closest_block = cube_rocks
                .iter()
                // Rocks that are on the same row
                .filter(|(_cube_x, cube_y)| cube_y == rock_y)
                // First blockage to the east of current rock
                .find(|(cube_x, _cube_y)| cube_x > rock_x)
                .copied()
                .map_or((*east_edge, *rock_y), |(block_x, block_y)| {
                    (block_x - 1, block_y)
                });
            let rocks_between_rock_and_block = round_rocks
                .iter()
                // Rocks that are on the same row
                .filter(|(_round_x, round_y)| round_y == rock_y)
                // Rocks that are farther than the blockage
                .filter(|(round_x, _round_y)| round_x <= &closest_block.0)
                // Rocks between blockage and current rock
                .filter(|(round_x, _round_y)| round_x > rock_x)
                .count();
            rolled.push((closest_block.0 - rocks_between_rock_and_block, *rock_y));
        }
        rolled
    } else {
        vec![]
    }
}

type RockLists = (Vec<(usize, usize)>, Vec<(usize, usize)>);
fn get_rock_locations(input: &str) -> RockLists {
    input
        .lines()
        .enumerate()
        .fold((vec![], vec![]), |(mut round, mut cube), (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| match c {
                '#' => cube.push((x, y)),
                'O' => round.push((x, y)),
                _ => (),
            });
            (round, cube)
        })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    const PART1_INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    const PART1_INPUT_ROLLED: &str = r#"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."#;

    const PART1_INPUT_CYCLE1: &str = r#".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."#;

    const PART1_INPUT_CYCLE2: &str = r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"#;

    const PART1_INPUT_CYCLE3: &str = r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"#;

    #[test]
    fn part1_test() -> Result<(), String> {
        let (round, cube) = get_rock_locations(PART1_INPUT);
        let (round_rolled, cube_rolled) = get_rock_locations(PART1_INPUT_ROLLED);
        let rolled = roll_rocks_north(&round, &cube);
        assert_eq!(cube, cube_rolled);
        assert_eq!(
            BTreeSet::from_iter(round_rolled),
            BTreeSet::from_iter(rolled)
        );

        assert_eq!(calculate_load_after_roll(PART1_INPUT), Ok(136));

        Ok(())
    }

    #[test]
    fn part2_test() -> Result<(), String> {
        let (round, cube) = get_rock_locations(PART1_INPUT);
        let mut rolled = round;
        for input in [PART1_INPUT_CYCLE1, PART1_INPUT_CYCLE2, PART1_INPUT_CYCLE3] {
            let (round_rolled, cube_rolled) = get_rock_locations(input);
            rolled = roll_cycle(&rolled, &cube);

            assert_eq!(cube, cube_rolled);
            assert_eq!(
                BTreeSet::from_iter(round_rolled),
                BTreeSet::from_iter(rolled.iter().copied())
            );
        }
        assert_eq!(calculate_load_after_roll_cycle(PART1_INPUT), Ok(64));

        Ok(())
    }
}
