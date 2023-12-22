use std::{collections::BTreeSet, io::Read};

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day21_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = garden_plots_reachable_in_n_steps(&input, 64);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = garden_plots_reachable_in_n_steps(&input, 26_501_365);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

type Garden = Vec<Vec<GardenTile>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GardenTile {
    Plot,
    Stone,
}

#[derive(Debug)]
struct EdgeBuffer {
    active: BTreeSet<(isize, isize)>,
    inactive: BTreeSet<(isize, isize)>,
    count: usize,
}

impl EdgeBuffer {
    pub fn new() -> Self {
        Self {
            active: BTreeSet::new(),
            inactive: BTreeSet::new(),
            count: 0,
        }
    }

    pub fn new_with_active(active: (isize, isize)) -> Self {
        Self {
            active: BTreeSet::from_iter([active]),
            inactive: BTreeSet::new(),
            count: 0,
        }
    }

    pub fn swap(&mut self) {
        self.count += self.inactive.len();
        self.inactive.clear();
    }

    pub fn push(&mut self, item: (isize, isize)) {
        if !self.inactive.contains(&item) {
            self.active.insert(item);
        }
    }

    pub fn pop(&mut self) -> Option<(isize, isize)> {
        if let Some(item) = self.active.pop_first() {
            self.inactive.insert(item);
            Some(item)
        } else {
            None
        }
    }

    pub fn count(&self) -> usize {
        self.count + self.active.len() + self.inactive.len()
    }
}

// From Day09
fn extrapolate_history_forward(history: &[usize]) -> Result<usize, String> {
    let diffs = std::iter::successors(Some(history.to_vec()), |prev| {
        if prev.iter().all(|diff| diff.eq(&0)) {
            None
        } else {
            Some(prev.windows(2).map(|wind| wind[1] - wind[0]).collect())
        }
    })
    .collect::<Vec<_>>();
    diffs
        .into_iter()
        .rev()
        .try_fold(0, |prev_diff, cur_diffs| match cur_diffs.last() {
            Some(cur_diff) => Ok(prev_diff + cur_diff),
            None => Err("A list of differences was empty.".to_owned()),
        })
}

fn garden_plots_reachable_in_n_steps(input: &str, steps: usize) -> Result<usize, String> {
    const STEPS_TO_STEADY_STATE: usize = 4;

    let (garden, start) = read_garden_map(input)?;

    let columns = garden[0].len();

    // If the number of grid widths you would walk is less than the number of widths to reach steady state (magic number) + 3, do brute force
    if steps / columns < (STEPS_TO_STEADY_STATE + 3) {
        plots_reachable_in_n_steps(&garden, start, steps)
    } else {
        let widths = steps / columns;
        let extra = steps % columns;

        let width_walk = [
            plots_reachable_in_n_steps(&garden, start, STEPS_TO_STEADY_STATE * columns + extra)?,
            plots_reachable_in_n_steps(
                &garden,
                start,
                (STEPS_TO_STEADY_STATE + 1) * columns + extra,
            )?,
            plots_reachable_in_n_steps(
                &garden,
                start,
                (STEPS_TO_STEADY_STATE + 2) * columns + extra,
            )?,
            plots_reachable_in_n_steps(
                &garden,
                start,
                (STEPS_TO_STEADY_STATE + 3) * columns + extra,
            )?,
        ];

        ((STEPS_TO_STEADY_STATE + 3)..widths)
            .try_fold(width_walk, |window, _| -> Result<[usize; 4], String> {
                Ok([
                    window[1],
                    window[2],
                    window[3],
                    extrapolate_history_forward(&window)?,
                ])
            })
            .map(|window| window[3])
    }
}

fn walk_garden(
    current_step: &mut EdgeBuffer,
    reachable: &mut EdgeBuffer,
    garden: &Garden,
) -> Result<(), String> {
    let rem_euclid_with_cast = |a: isize, modulus: usize| -> Result<usize, String> {
        usize::try_from(
            a.rem_euclid(
                isize::try_from(modulus)
                    .map_err(|_err| "Trying to cast an usize to isize with wrapping.".to_owned())?,
            ),
        )
        .map_err(|_err| "Trying to cast an isize to usize with wrapping.".to_owned())
    };

    let rows = garden.len();
    let columns = garden[0].len();

    while let Some((tile_x, tile_y)) = current_step.pop() {
        [
            (tile_x - 1, tile_y),
            (tile_x + 1, tile_y),
            (tile_x, tile_y - 1),
            (tile_x, tile_y + 1),
        ]
        .into_iter()
        .try_for_each(|(x, y)| -> Result<(), String> {
            if garden[rem_euclid_with_cast(y, rows)?][rem_euclid_with_cast(x, columns)?]
                == GardenTile::Plot
            {
                reachable.push((x, y));
            }

            Ok(())
        })?;
    }

    std::mem::swap(current_step, reachable);
    current_step.swap();

    Ok(())
}

fn plots_reachable_in_n_steps(
    garden: &Garden,
    start: (isize, isize),
    steps: usize,
) -> Result<usize, String> {
    let mut buffer_1 = EdgeBuffer::new_with_active(start);
    let mut buffer_2 = EdgeBuffer::new();

    for _step in 0..steps {
        walk_garden(&mut buffer_1, &mut buffer_2, garden)?;
    }

    Ok(buffer_1.count())
}

fn read_garden_map(input: &str) -> Result<(Garden, (isize, isize)), String> {
    input
        .lines()
        .enumerate()
        .try_fold((vec![], (0, 0)), |(mut rows, pos), (y, line)| {
            let mut start = None;

            let cur_row = line
                .chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Ok(GardenTile::Plot),
                    '#' => Ok(GardenTile::Stone),
                    'S' => {
                        let _ = start.insert((
                            isize::try_from(x).map_err(|err| err.to_string())?,
                            isize::try_from(y).map_err(|err| err.to_string())?,
                        ));
                        Ok(GardenTile::Plot)
                    }
                    c => Err(format!("Unknow character '{c}' found in '({x}, {y})'")),
                })
                .collect::<Result<Vec<_>, _>>()?;
            rows.push(cur_row);

            if let Some(start) = start {
                Ok((rows, start))
            } else {
                Ok((rows, pos))
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_INPUT: &str = r"...........
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

    #[test]
    fn part1_test() {
        assert_eq!(garden_plots_reachable_in_n_steps(PART1_INPUT, 6), Ok(16));
    }

    #[test]
    fn part2_test() {
        assert_eq!(garden_plots_reachable_in_n_steps(PART1_INPUT, 10), Ok(50));
        assert_eq!(garden_plots_reachable_in_n_steps(PART1_INPUT, 50), Ok(1594));
        assert_eq!(
            garden_plots_reachable_in_n_steps(PART1_INPUT, 100),
            Ok(6536)
        );
        assert_eq!(
            garden_plots_reachable_in_n_steps(PART1_INPUT, 500),
            Ok(167004)
        );
        assert_eq!(
            garden_plots_reachable_in_n_steps(PART1_INPUT, 1000),
            Ok(668697)
        );
        // assert_eq!(
        //     garden_plots_reachable_in_n_steps(PART1_INPUT, 5000),
        //     Ok(16733044)
        // );
    }
}
