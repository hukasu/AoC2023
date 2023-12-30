mod hailstone;

use std::io::Read;

use crate::hailstone::Hailstone;

const BOUND_MIN: f64 = 200_000_000_000_000.;
const BOUND_MAX: f64 = 400_000_000_000_000.;

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day24_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = find_trajectory_intersections(&input, (BOUND_MIN, BOUND_MAX));
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = find_throw_that_hits_all(&input, (BOUND_MIN, BOUND_MAX));
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn find_trajectory_intersections(input: &str, bounds: (f64, f64)) -> Result<usize, String> {
    let hailstones = read_hailstorm_snapshot(input)?;
    Ok(find_intersection(&hailstones, bounds))
}

fn find_throw_that_hits_all(input: &str, bounds: (f64, f64)) -> Result<i64, String> {
    let hailstones = read_hailstorm_snapshot(input)?;
    let (x, y, z) = find_perfect_throw(&hailstones, bounds)?.position;
    Ok(x as i64 + y as i64 + z as i64)
}

fn calculate_total_distance(hailstones: &[Hailstone], throw: &Hailstone) -> f64 {
    hailstones
        .iter()
        .map(|stone| throw.skew_line_distance(stone).powi(2))
        .sum::<f64>()
}

fn find_perfect_throw(hailstones: &[Hailstone], bounds: (f64, f64)) -> Result<Hailstone, String> {
    const STEP: f64 = 10_000_000_000.;
    let perfect_throw = if let (Some(first), Some(last)) = (hailstones.first(), hailstones.last()) {
        let first_in_bounds = first.time_in_bounds(bounds);
        let last_in_bounds = last.time_in_bounds(bounds);

        if first_in_bounds.1 - first_in_bounds.0 < STEP * 10. {
            find_candidate_in_range(
                hailstones,
                first,
                first_in_bounds,
                last,
                last_in_bounds,
                bounds,
            )
        } else {
            let mut hailstone: Option<Hailstone> = None;
            std::thread::scope(|scope| {
                let (in_channel, out_channel) = std::sync::mpsc::channel();
                for (s, e) in std::iter::successors(
                    Some((first_in_bounds.0, first_in_bounds.0 + STEP)),
                    |(_, e)| {
                        if e + STEP > first_in_bounds.1 {
                            None
                        } else {
                            Some((*e, e + STEP))
                        }
                    },
                ) {
                    let in_clone = in_channel.clone();
                    scope.spawn(move || {
                        let _ = in_clone.send(find_candidate_in_range(
                            hailstones,
                            first,
                            (s, e),
                            last,
                            last_in_bounds,
                            bounds,
                        ));
                    });
                }

                // Drop original channel to prevent blocking
                std::mem::drop(in_channel);

                while let Ok(out) = out_channel.recv() {
                    match out {
                        Ok(throw) => {
                            let stored = hailstone.take();
                            let new_min = if let Some(stored_throw) = stored {
                                if calculate_total_distance(hailstones, &throw)
                                    < calculate_total_distance(hailstones, &stored_throw)
                                {
                                    throw
                                } else {
                                    stored_throw
                                }
                            } else {
                                throw
                            };
                            hailstone.replace(new_min);
                        }
                        Err(err) => println!("{err}"),
                    }
                }
            });

            hailstone.ok_or("Failed to find throw.".to_owned())
        }
    } else {
        Err("Failed to get a hailstone.".to_owned())?
    };
    perfect_throw
}

fn find_candidate_in_range(
    hailstones: &[Hailstone],
    first_hailstone: &Hailstone,
    first_hailstone_range: (f64, f64),
    second_hailstone: &Hailstone,
    second_hailstone_range: (f64, f64),
    bounds: (f64, f64),
) -> Result<Hailstone, String> {
    const POWER_OF_TEN_DECREASE: f64 = 3.;

    let get_best_candidate = |iter: Box<dyn Iterator<Item = (f64, f64)>>| {
        iter.map(|(llr, rlr)| {
            (
                llr,
                rlr,
                Hailstone::new_between_interpolated_hailstones(
                    first_hailstone,
                    llr,
                    second_hailstone,
                    rlr,
                ),
            )
        })
        .filter(|(_, _, throw)| throw.in_bounds(bounds))
        .map(|(llr, rlr, throw)| {
            let dist = calculate_total_distance(hailstones, &throw);
            (llr, rlr, dist)
        })
        .min_by(|(_, _, l), (_, _, r)| l.total_cmp(r))
        .ok_or("Failed to get starting candidate.")
    };

    let hailstone1_iter = {
        let first_range_step_power = ((first_hailstone_range.1 - first_hailstone_range.0).log10()
            - POWER_OF_TEN_DECREASE)
            .max(0.) as u32;
        ((first_hailstone_range.0 as i64)..=(first_hailstone_range.1 as i64))
            .step_by(10usize.pow(first_range_step_power))
            .map(|t| t as f64)
    };
    let hailstone2_iter = {
        let last_range_step_power = ((second_hailstone_range.1 - second_hailstone_range.0).log10()
            - POWER_OF_TEN_DECREASE)
            .max(0.) as u32;
        ((second_hailstone_range.0 as i64)..=(second_hailstone_range.1 as i64))
            .step_by(10usize.pow(last_range_step_power))
            .map(|t| t as f64)
    };

    let start_iter: Box<dyn Iterator<Item = (f64, f64)>> = Box::new(
        hailstone1_iter.flat_map(|t| [t].into_iter().cycle().zip(hailstone2_iter.clone())),
    );

    let (mut range1, mut range2, mut error) = get_best_candidate(start_iter)?;
    let mut learning_rate = 1_000.;

    while error > 0. {
        let candidate = get_best_candidate(Box::new(
            [
                (range1, range2),
                (range1, range2 + learning_rate),
                (range1, range2 - learning_rate),
                (range1 + learning_rate, range2),
                (range1 + learning_rate, range2 + learning_rate),
                (range1 + learning_rate, range2 - learning_rate),
                (range1 - learning_rate, range2),
                (range1 - learning_rate, range2 + learning_rate),
                (range1 - learning_rate, range2 - learning_rate),
            ]
            .into_iter(),
        ))?;

        if candidate.0 < first_hailstone_range.0
            || candidate.0 > first_hailstone_range.1
            || candidate.1 < second_hailstone_range.0
            || candidate.1 > second_hailstone_range.1
        {
            println!("Out of bounds.");
            break;
        } else if (range1 - candidate.0).abs() < f64::EPSILON
            && (range2 - candidate.1).abs() < f64::EPSILON
        {
            if learning_rate > 1. {
                learning_rate /= 10.;
            } else {
                println!("Stagnated.");
                break;
            }
            error = candidate.2;
        } else {
            range1 = candidate.0;
            range2 = candidate.1;
            error = candidate.2;
        }
    }

    let perfect_throw = Hailstone::new_between_interpolated_hailstones(
        first_hailstone,
        range1,
        second_hailstone,
        range2,
    );

    Ok(perfect_throw.move_hailstone(-(range1.min(range2))))
}

fn find_intersection(hailstones: &[Hailstone], bounds: (f64, f64)) -> usize {
    match hailstones {
        [] => 0,
        [head, tail @ ..] => {
            tail.iter()
                .filter(|other| {
                    let (inter, timings) = head.intersection_2d(other);
                    (inter.0 > bounds.0 && inter.0 < bounds.1)
                        && (inter.1 > bounds.0 && inter.1 < bounds.1)
                        && timings.0 >= 0.
                        && timings.1 >= 0.
                })
                .count()
                + find_intersection(tail, bounds)
        }
    }
}

fn read_hailstorm_snapshot(input: &str) -> Result<Vec<Hailstone>, String> {
    input
        .lines()
        .map(|line| {
            match line
                .replace('@', ",")
                .split(',')
                .map(|i| i.trim().parse::<f64>())
                .collect::<Result<Vec<_>, std::num::ParseFloatError>>()
                .map_err(|err| format!("Failed to parse hailstone string. '{err}'"))?
                .as_slice()
            {
                [x, y, z, vx, vy, vz] => Ok(Hailstone::new((*x, *y, *z), (*vx, *vy, *vz))),
                _ => Err("Failed to parse hailstone.".to_owned()),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_INPUT: &str = r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn part1_input() {
        assert_eq!(find_trajectory_intersections(PART1_INPUT, (7., 27.)), Ok(2));
    }

    #[test]
    fn part2_input() {
        assert_eq!(find_throw_that_hits_all(PART1_INPUT, (7., 27.)), Ok(47));
    }
}
