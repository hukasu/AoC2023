use std::io::Read;

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day11_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = find_paths_between_galaxies(&input, 2);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = find_paths_between_galaxies(&input, 1_000_000);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn find_paths_between_galaxies(input: &str, rate_of_expansion: usize) -> Result<u64, String> {
    let make_pairs = |v: &[(usize, usize)]| -> Vec<((usize, usize), (usize, usize))> {
        match v {
            [source, tail @ ..] => tail.iter().map(|dest| (*source, *dest)).collect(),
            [] => vec![],
        }
    };
    let expanded_universe = get_expanded_column_rows(input)?;
    let galaxy_coordinates = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| if c == '#' { Some((x, y)) } else { None })
                .map(|(x, y)| {
                    (
                        expanded_universe
                            .0
                            .iter()
                            .filter(|expanded_x| expanded_x < &&x)
                            .count()
                            * (rate_of_expansion - 1)
                            + x,
                        y,
                    )
                })
                .collect::<Vec<_>>()
        })
        .map(|(x, y)| {
            (
                x,
                expanded_universe
                    .1
                    .iter()
                    .filter(|expanded_y| expanded_y < &&y)
                    .count()
                    * (rate_of_expansion - 1)
                    + y,
            )
        })
        .collect::<Vec<_>>();
    let galaxy_pairs = std::iter::successors(Some(galaxy_coordinates.as_slice()), |slice| {
        if slice.len() > 1 {
            Some(&slice[1..])
        } else {
            None
        }
    })
    .flat_map(make_pairs)
    .collect::<Vec<_>>();
    galaxy_pairs
        .into_iter()
        .map(|(source, dest)| source.0.abs_diff(dest.0) + source.1.abs_diff(dest.1))
        .map(|dist| {
            u64::try_from(dist)
                .map_err(|err| format!("Failed to convert from usize to u64. '{err}'"))
        })
        .sum()
}

fn get_expanded_column_rows(input: &str) -> Result<(Vec<usize>, Vec<usize>), String> {
    let line_length = input.lines().next().ok_or("Empty imput")?.len();
    let rows = input
        .lines()
        .enumerate()
        .filter_map(|(y, line)| {
            if line.chars().all(|c| c == '.') {
                Some(y)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let columns = (0..line_length)
        .filter(|col| {
            input
                .lines()
                .all(|line| line.chars().nth(*col).filter(|c| c == &'.').is_some())
        })
        .collect();

    Ok((columns, rows))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_INPUT1: &str = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part1_test() {
        assert_eq!(
            get_expanded_column_rows(PART1_INPUT1),
            Ok((vec![2, 5, 8], vec![3, 7]))
        );
        assert_eq!(find_paths_between_galaxies(PART1_INPUT1, 2), Ok(374));
    }

    #[test]
    fn part2_test() {
        assert_eq!(find_paths_between_galaxies(PART1_INPUT1, 10), Ok(1030));
        assert_eq!(find_paths_between_galaxies(PART1_INPUT1, 100), Ok(8410));
    }
}
