mod brick;

use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    io::Read,
};

use crate::brick::Brick;

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day22_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = count_safe_to_disentegrate(&input);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = gravity_assisted_disintegration(&input);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn count_safe_to_disentegrate(input: &str) -> Result<usize, String> {
    let settled = settle_bricks(read_brick_snapshot(input)?);
    let supporting = mark_bricks_that_are_required_support(&settled);

    Ok(settled.len() - supporting.len())
}

fn gravity_assisted_disintegration(input: &str) -> Result<usize, String> {
    let settled = settle_bricks(read_brick_snapshot(input)?);
    let bricks_top_and_bottom = get_sitting_on_top_and_bottom(&settled);

    let falling: usize = bricks_top_and_bottom
        .iter()
        .map(|(brick, (_below_brick, on_top_of_brick))| {
            let mut queue = VecDeque::from_iter(on_top_of_brick.iter().copied().filter(|other| {
                bricks_top_and_bottom
                    .get(*other)
                    .filter(|(bricks_on_bottom, _bricks_on_top)| bricks_on_bottom.len() == 1)
                    .is_some()
            }));

            let mut falling = BTreeSet::from_iter(queue.iter().copied());
            falling.insert(*brick);

            while let Some(cur) = queue.pop_front() {
                if let Some((_below_cur, on_top_of_cur)) = bricks_top_and_bottom.get(cur) {
                    for other in on_top_of_cur
                        .iter()
                        .filter(|other| !falling.contains(*other))
                        .collect::<Vec<_>>()
                    {
                        if let Some((below_other, _on_top_of_other)) =
                            bricks_top_and_bottom.get(other)
                        {
                            if falling.is_superset(below_other) {
                                queue.push_back(other);
                                falling.insert(other);
                            }
                        }
                    }
                }
            }

            falling.len() - 1
        })
        .sum();

    Ok(falling)
}

fn settle_bricks(bricks: BTreeSet<Brick>) -> BTreeSet<Brick> {
    let mut buffer: BTreeSet<Brick> = BTreeSet::new();

    for brick in bricks {
        let mut gravitated = brick;
        while !gravitated.on_ground()
            && !buffer
                .iter()
                .rev()
                .any(|other: &Brick| other.collision_detection(&gravitated.apply_gravity()))
        {
            gravitated = gravitated.apply_gravity();
        }

        buffer.insert(gravitated);
    }

    buffer
}

fn get_sitting_on_top_and_bottom(
    bricks: &BTreeSet<Brick>,
) -> BTreeMap<&Brick, (BTreeSet<&Brick>, BTreeSet<&Brick>)> {
    bricks
        .iter()
        .map(|brick| {
            let bottom = {
                let gravitated = brick.apply_gravity();
                bricks
                    .iter()
                    .filter(|other| brick != *other)
                    .filter(|other| gravitated.collision_detection(other))
                    .collect()
            };
            let top = bricks
                .iter()
                .filter(|other| brick != *other)
                .filter(|other| {
                    let gravitated = other.apply_gravity();
                    brick.collision_detection(&gravitated)
                })
                .collect();
            (brick, (bottom, top))
        })
        .collect()
}

fn mark_bricks_that_are_required_support(bricks: &BTreeSet<Brick>) -> BTreeSet<&Brick> {
    bricks
        .iter()
        .filter_map(|brick| {
            if !brick.on_ground() {
                let gravitated = brick.apply_gravity();
                if let [single_detection] = bricks
                    .iter()
                    .filter(|other| *other != brick)
                    .filter(|other| other.collision_detection(&gravitated))
                    .collect::<Vec<_>>()
                    .as_slice()
                {
                    Some(*single_detection)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn read_brick_snapshot(input: &str) -> Result<BTreeSet<Brick>, String> {
    input
        .lines()
        .map(|line| {
            if let Some((left, right)) = line.split_once('~') {
                let l_end = if let [x, y, z] = left.split(',').collect::<Vec<_>>().as_slice() {
                    (
                        x.parse::<usize>().map_err(|err| err.to_string())?,
                        y.parse::<usize>().map_err(|err| err.to_string())?,
                        z.parse::<usize>().map_err(|err| err.to_string())?,
                    )
                } else {
                    Err("Failed to parse coordinates.")?
                };
                let r_end = if let [x, y, z] = right.split(',').collect::<Vec<_>>().as_slice() {
                    (
                        x.parse::<usize>().map_err(|err| err.to_string())?,
                        y.parse::<usize>().map_err(|err| err.to_string())?,
                        z.parse::<usize>().map_err(|err| err.to_string())?,
                    )
                } else {
                    Err("Failed to parse coordinates.")?
                };
                let ((lx, ly, lz), (rx, ry, rz)) = (l_end, r_end);

                Ok(Brick {
                    base_x: lx.min(rx),
                    base_y: ly.min(ry),
                    base_z: lz.min(rz),
                    base_w: lx.abs_diff(rx) + 1,
                    base_d: ly.abs_diff(ry) + 1,
                    height: lz.abs_diff(rz) + 1,
                })
            } else {
                Err(format!("Brick coordinate is malformed. '{line}'"))
            }
        })
        .collect::<Result<BTreeSet<_>, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_INPUT: &str = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn part1_test() {
        assert_eq!(count_safe_to_disentegrate(PART1_INPUT), Ok(5));
    }

    #[test]
    fn part2_test() {
        assert_eq!(gravity_assisted_disintegration(PART1_INPUT), Ok(7));
    }
}
