use std::io::Read;

#[derive(Debug)]
struct MapInstruction<'a> {
    label: &'a str,
    left: &'a str,
    right: &'a str,
}

enum InstructionSide {
    Left,
    Right,
}

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day08_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = follow_map(&input);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = follow_ghost_map(&input);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn process_input(input: &str) -> Result<(Vec<InstructionSide>, Vec<MapInstruction>), String> {
    let mut lines = input.lines();
    let instrunction_order = {
        let instructions = lines.next().ok_or("Input was empty.".to_owned())?;
        instructions
            .chars()
            .map(|c| match c {
                'L' => Ok(InstructionSide::Left),
                'R' => Ok(InstructionSide::Right),
                _ => Err("Failed to decode instruction order.".to_owned()),
            })
            .collect::<Result<Vec<_>, String>>()?
    };

    if lines
        .next()
        .ok_or("Input reached EOF before list on instructions.".to_owned())?
        .is_empty()
    {
        let mut instructions = lines
            .map(|line| {
                let slices = std::iter::successors(Some(line), |next| {
                    if next.len() < 4 {
                        None
                    } else {
                        Some(&next[1..])
                    }
                })
                .map(|slice| &slice[..3])
                .filter(|slice| slice.chars().all(|c| c.is_ascii_alphanumeric()))
                .collect::<Vec<_>>();
                if let [label, left, right] = slices.as_slice() {
                    Ok(MapInstruction { label, left, right })
                } else {
                    Err(format!("Malformatted line. '{line}'"))
                }
            })
            .collect::<Result<Vec<_>, String>>()?;
        instructions.sort_by_key(|inst| inst.label);
        Ok((instrunction_order, instructions))
    } else {
        Err("There was no space between order of instructions and list of instructions.".to_owned())
    }
}

fn next_node<'a>(node: &str, instructions: &'a [MapInstruction]) -> Option<&'a MapInstruction<'a>> {
    match instructions.binary_search_by_key(&node, |inst| inst.label) {
        Ok(idx) => instructions.get(idx),
        Err(_) => None,
    }
}

fn follow_map(input: &str) -> Result<u64, String> {
    const START: &str = "AAA";
    const END: &str = "ZZZ";

    let (instruction_order, instructions) = process_input(input)?;

    let start = next_node(START, &instructions).ok_or("Could not find start node.".to_owned())?;

    follow_instructios(
        start,
        &instructions,
        &instruction_order,
        &|_steps: u64, node: &MapInstruction| node.label.ne(END),
    )
}

fn follow_instructios(
    start: &MapInstruction,
    instructions: &[MapInstruction],
    instruction_order: &[InstructionSide],
    test: &dyn Fn(u64, &MapInstruction) -> bool,
) -> Result<u64, String> {
    let mut instruction_order_cycle = instruction_order.iter().cycle();
    let mut node = start;
    let mut steps = 0;
    while test(steps, node) {
        steps += 1;
        let next = match instruction_order_cycle.next() {
            Some(InstructionSide::Left) => node.left,
            Some(InstructionSide::Right) => node.right,
            None => Err("Cycle returned None.")?,
        };
        node = next_node(next, instructions).ok_or(format!("Could not find node '{next}'."))?;
    }
    Ok(steps)
}

fn follow_ghost_map(input: &str) -> Result<u64, String> {
    const PRIMES: &[u64; 60] = &[
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
    ];
    let (instruction_order, instructions) = process_input(input)?;

    // Assumption
    // All start and end pairs are closed cycles
    let mut cycles = instructions
        .iter()
        .filter(|inst| inst.label.ends_with('A'))
        .map(|start| {
            follow_instructios(
                start,
                &instructions,
                &instruction_order,
                &|steps: u64, node: &MapInstruction| steps == 0 || !node.label.ends_with('Z'),
            )
        })
        .collect::<Result<Vec<_>, String>>()?;

    let mut factors = vec![];
    for prime in PRIMES {
        while !&cycles.iter().all(|i| i.eq(&1)) {
            let mut divided_any = false;
            for c in &mut cycles {
                if *c % prime == 0 {
                    *c /= prime;
                    divided_any = true;
                }
            }
            if divided_any {
                factors.push(prime);
            } else {
                break;
            }
        }
    }
    Ok(factors.into_iter().product())
}

#[cfg(test)]
mod tests {
    const PART1_INPUT1: &str = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const PART1_INPUT2: &str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const PART2_INPUT1: &str = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part1_test() {
        assert_eq!(super::follow_map(PART1_INPUT1), Ok(2));
        assert_eq!(super::follow_map(PART1_INPUT2), Ok(6));
    }

    #[test]
    fn part2_test() {
        assert_eq!(super::follow_ghost_map(PART1_INPUT1), Ok(2));
        assert_eq!(super::follow_ghost_map(PART1_INPUT2), Ok(6));
        assert_eq!(super::follow_ghost_map(PART2_INPUT1), Ok(6));
    }
}
