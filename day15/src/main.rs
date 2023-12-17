mod hash;

use std::io::Read;

use hash::hash;

#[derive(Debug, Clone)]
struct LensBox<'a>(pub Vec<Lens<'a>>);

#[derive(Debug, Clone)]
struct Lens<'a> {
    pub label: &'a str,
    pub focal_length: u8,
}

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day15_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = initialization_sequence_sum(&input);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = lenses_focusing_power(&input);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn initialization_sequence_sum(input: &str) -> Result<u64, String> {
    input
        .lines()
        .flat_map(|line| line.split(','))
        .map(hash)
        .map(|res| {
            res.map(u64::from)
                .map_err(|err| format!("Hash failed with '{err}'."))
        })
        .sum()
}

fn lenses_focusing_power(input: &str) -> Result<u64, String> {
    let instructions = input
        .lines()
        .flat_map(|line| line.split(','))
        .collect::<Vec<_>>();

    let mut boxes = vec![LensBox(vec![]); 256];

    for inst in instructions {
        if inst.ends_with('-') {
            let hash =
                hash::hash(inst.trim_end_matches('-')).map_err(|err| err.to_string())? as usize;
            let relevant_box = &mut boxes[hash].0;
            if let Some(pos) = relevant_box
                .iter()
                .position(|lens_box| lens_box.label.eq(inst.trim_end_matches('-')))
            {
                relevant_box.remove(pos);
            }
        } else if let Some((label, focal_length)) = inst.split_once('=') {
            let hash = hash::hash(label).map_err(|err| err.to_string())? as usize;
            let focal_length = focal_length
                .parse()
                .map_err(|err| format!("Could not parse focal length. '{err}'"))?;

            let relevant_box = &mut boxes[hash].0;
            if let Some(pos) = relevant_box
                .iter()
                .position(|lens_box| lens_box.label.eq(label))
            {
                relevant_box[pos].focal_length = focal_length;
            } else {
                relevant_box.push(Lens {
                    label,
                    focal_length,
                });
            }
        } else {
            Err(format!("Malformed instruction.'{inst}'"))?;
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(box_id, lens_box)| {
            lens_box
                .0
                .iter()
                .enumerate()
                .map(|(lens_id, lens)| {
                    [(box_id + 1), (lens_id + 1), lens.focal_length as usize]
                        .into_iter()
                        .map(|val| {
                            u64::try_from(val).map_err(|err| format!("Hash failed with '{err}'."))
                        })
                        .product::<Result<u64, String>>()
                })
                .sum::<Result<u64, String>>()
        })
        .collect::<Vec<_>>()
        .into_iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash::hash("HASH"), Ok(52));
    }

    #[test]
    fn part1_test() {
        assert_eq!(initialization_sequence_sum(PART1_INPUT), Ok(1320));
    }

    #[test]
    fn part2_test() {
        assert_eq!(lenses_focusing_power(PART1_INPUT), Ok(145));
    }
}
