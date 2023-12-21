mod workflow;

use std::io::Read;

use crate::workflow::{Part, Workflow, WorkflowResponse};

const MIN_RATING: u64 = 1;
const MAX_RATING: u64 = 4000;

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day19_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = get_usable_parts(&input);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = get_valid_combinations(&input);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn get_usable_parts(input: &str) -> Result<u64, String> {
    if let Some((workflows, parts)) = input.split_once("\n\n") {
        let workflows = workflows
            .lines()
            .map(Workflow::try_from)
            .collect::<Result<Vec<_>, String>>()?;
        let parts = parts
            .lines()
            .map(Part::try_from)
            .collect::<Result<Vec<_>, String>>()?;

        parts
            .iter()
            .filter_map(|part| {
                let mut workflow_name = "in";
                loop {
                    match workflows
                        .iter()
                        .find(|workflow| workflow.name == workflow_name)
                        .ok_or("Workflow does not exist.")
                    {
                        Ok(workflow) => match workflow.run_part_through_workflow(part) {
                            Ok(WorkflowResponse::Accept) => break Some(Ok(part)),
                            Ok(WorkflowResponse::Reject) => break None,
                            Ok(WorkflowResponse::NextWorkflow(next)) => workflow_name = next,
                            Err(err) => break Some(Err(err)),
                        },
                        Err(err) => break Some(Err(err.to_owned())),
                    }
                }
            })
            .map(|maybe_part| match maybe_part {
                Ok(part) => Ok(part.value()),
                Err(err) => Err(err),
            })
            .sum()
    } else {
        Err("Malformed input.".to_owned())
    }
}

fn get_valid_combinations(input: &str) -> Result<u64, String> {
    if let Some((workflows, _parts)) = input.split_once("\n\n") {
        let workflows = workflows
            .lines()
            .map(Workflow::try_from)
            .collect::<Result<Vec<_>, String>>()?;

        workflows
            .iter()
            .find(|workflow| workflow.name == "in")
            .map_or(
                Err("Failed to calculate valid combinations.".to_owned()),
                |in_workflow| {
                    in_workflow.valid_combinations(
                        &workflows,
                        (
                            MIN_RATING..=MAX_RATING,
                            MIN_RATING..=MAX_RATING,
                            MIN_RATING..=MAX_RATING,
                            MIN_RATING..=MAX_RATING,
                        ),
                    )
                },
            )
    } else {
        Err("Malformed input.".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_INPUT: &str = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn part1_test() {
        assert_eq!(get_usable_parts(PART1_INPUT), Ok(19114));
    }

    #[test]
    fn part2_test() {
        assert_eq!(get_valid_combinations(PART1_INPUT), Ok(167_409_079_868_000));
    }
}
