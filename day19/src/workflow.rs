use std::ops::RangeInclusive;

#[derive(Debug, Default)]
pub struct Part {
    cool: u64,
    musical: u64,
    aerodynamic: u64,
    shiny: u64,
}

impl Part {
    pub fn value(&self) -> u64 {
        self.cool + self.musical + self.aerodynamic + self.shiny
    }
}

impl TryFrom<&str> for Part {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .try_fold(
                Self::default(),
                |mut part, rating_component| -> Result<Self, Self::Error> {
                    if let Some((rating_type, rating_str)) = rating_component.split_once('=') {
                        let rating = rating_str
                            .parse::<u64>()
                            .map_err(|err| format!("Failed to parse Part rating. '{err}'"))?;
                        match rating_type {
                            "x" => part.cool = rating,
                            "m" => part.musical = rating,
                            "a" => part.aerodynamic = rating,
                            "s" => part.shiny = rating,
                            t => Err(format!("Unknown rating type. '{t}'"))?,
                        }
                        Ok(part)
                    } else {
                        Err("Failed to parse Part.".to_owned())
                    }
                },
            )
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum WorkflowResponse<'a> {
    Accept,
    Reject,
    NextWorkflow(&'a str),
}

#[derive(Debug)]
pub enum Rating {
    Cool(u64),
    Musical(u64),
    Aerodynamic(u64),
    Shiny(u64),
}

#[derive(Debug)]
pub struct Rule<'a> {
    rating: Rating,
    response: &'a str,
    operand: std::cmp::Ordering,
}

pub type WorkingRanges = (
    RangeInclusive<u64>,
    RangeInclusive<u64>,
    RangeInclusive<u64>,
    RangeInclusive<u64>,
);

#[derive(Debug)]
pub struct Workflow<'a> {
    pub name: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    pub fn run_part_through_workflow(&'a self, part: &Part) -> Result<WorkflowResponse, String> {
        match self.rules.iter().find_map(|rule| {
            let Rule {
                rating,
                response,
                operand: rule_operand,
            } = rule;

            let comparison = match rating {
                Rating::Cool(c) => part.cool.cmp(c),
                Rating::Musical(m) => part.musical.cmp(m),
                Rating::Aerodynamic(a) => part.aerodynamic.cmp(a),
                Rating::Shiny(s) => part.shiny.cmp(s),
            };
            match comparison {
                op if op == *rule_operand => Some(Ok(match response {
                    &"A" => WorkflowResponse::Accept,
                    &"R" => WorkflowResponse::Reject,
                    next => WorkflowResponse::NextWorkflow(next),
                })),
                _ => None,
            }
        }) {
            Some(res) => res,
            None => Err("Failed to find next workflow.".to_owned()),
        }
    }

    pub fn valid_combinations(
        &self,
        worflows: &[Workflow],
        working_ranges: WorkingRanges,
    ) -> Result<u64, String> {
        self.rules
            .iter()
            .scan(working_ranges, |ranges, rule| {
                Some(match rule.rating {
                    Rating::Cool(c) => Self::valid_cool_combinations(rule, worflows, ranges, c),
                    Rating::Musical(m) => {
                        Self::valid_musical_combinations(rule, worflows, ranges, m)
                    }
                    Rating::Aerodynamic(a) => {
                        Self::valid_aerodynamic_combinations(rule, worflows, ranges, a)
                    }
                    Rating::Shiny(s) => Self::valid_shiny_combinations(rule, worflows, ranges, s),
                })
            })
            .sum()
    }

    fn valid_cool_combinations(
        rule: &Rule,
        workflows: &[Workflow],
        ranges: &mut WorkingRanges,
        cutoff: u64,
    ) -> Result<u64, String> {
        let Rule {
            rating: _,
            response,
            operand,
        } = rule;

        let cool_range = ranges.0.clone();
        if cutoff == u64::MAX {
            Self::validate_response(response, workflows, ranges.clone())
        } else if cool_range.contains(&cutoff) {
            let (negative_range, positive_range) =
                Self::generate_new_ranges(*operand, cutoff, cool_range)?;
            let positive_ranges = (
                positive_range,
                ranges.1.clone(),
                ranges.2.clone(),
                ranges.3.clone(),
            );
            *ranges = (
                negative_range,
                ranges.1.clone(),
                ranges.2.clone(),
                ranges.3.clone(),
            );

            Self::validate_response(response, workflows, positive_ranges)
        } else {
            Err("Outside0".to_owned())
        }
    }

    fn valid_musical_combinations(
        rule: &Rule,
        workflows: &[Workflow],
        ranges: &mut WorkingRanges,
        cutoff: u64,
    ) -> Result<u64, String> {
        let Rule {
            rating: _,
            response,
            operand,
        } = rule;

        let musical_range = ranges.1.clone();
        if cutoff == u64::MAX {
            Self::validate_response(response, workflows, ranges.clone())
        } else if musical_range.contains(&cutoff) {
            let (negative_range, positive_range) =
                Self::generate_new_ranges(*operand, cutoff, musical_range)?;
            let positive_ranges = (
                ranges.0.clone(),
                positive_range,
                ranges.2.clone(),
                ranges.3.clone(),
            );
            *ranges = (
                ranges.0.clone(),
                negative_range,
                ranges.2.clone(),
                ranges.3.clone(),
            );

            Self::validate_response(response, workflows, positive_ranges)
        } else {
            Err("Outside1".to_owned())
        }
    }

    fn valid_aerodynamic_combinations(
        rule: &Rule,
        workflows: &[Workflow],
        ranges: &mut WorkingRanges,
        cutoff: u64,
    ) -> Result<u64, String> {
        let Rule {
            rating: _,
            response,
            operand,
        } = rule;

        let aerodynamic_range = ranges.2.clone();
        if cutoff == u64::MAX {
            Self::validate_response(response, workflows, ranges.clone())
        } else if aerodynamic_range.contains(&cutoff) {
            let (negative_range, positive_range) =
                Self::generate_new_ranges(*operand, cutoff, aerodynamic_range)?;
            let positive_ranges = (
                ranges.0.clone(),
                ranges.1.clone(),
                positive_range,
                ranges.3.clone(),
            );
            *ranges = (
                ranges.0.clone(),
                ranges.1.clone(),
                negative_range,
                ranges.3.clone(),
            );

            Self::validate_response(response, workflows, positive_ranges)
        } else {
            Err("Outside2".to_owned())
        }
    }

    fn valid_shiny_combinations(
        rule: &Rule,
        workflows: &[Workflow],
        ranges: &mut WorkingRanges,
        cutoff: u64,
    ) -> Result<u64, String> {
        let Rule {
            rating: _,
            response,
            operand,
        } = rule;

        let shiny_range = ranges.3.clone();
        if cutoff == u64::MAX {
            Self::validate_response(response, workflows, ranges.clone())
        } else if shiny_range.contains(&cutoff) {
            let (negative_range, positive_range) =
                Self::generate_new_ranges(*operand, cutoff, shiny_range)?;
            let positive_ranges = (
                ranges.0.clone(),
                ranges.1.clone(),
                ranges.2.clone(),
                positive_range,
            );
            *ranges = (
                ranges.0.clone(),
                ranges.1.clone(),
                ranges.2.clone(),
                negative_range,
            );

            Self::validate_response(response, workflows, positive_ranges)
        } else {
            Err("Outside3".to_owned())
        }
    }

    fn validate_response(
        response: &str,
        worflows: &[Workflow],
        ranges: WorkingRanges,
    ) -> Result<u64, String> {
        match response {
            "A" => Ok([
                (ranges.0.end() - ranges.0.start()) + 1,
                (ranges.1.end() - ranges.1.start()) + 1,
                (ranges.2.end() - ranges.2.start()) + 1,
                (ranges.3.end() - ranges.3.start()) + 1,
            ]
            .into_iter()
            .product()),
            "R" => Ok(0),
            next => {
                if let Some(next_workflow) = worflows.iter().find(|workflow| workflow.name == next)
                {
                    next_workflow.valid_combinations(worflows, ranges)
                } else {
                    Err(format!("Failed to find next workflow. '{response}'"))
                }
            }
        }
    }

    fn generate_new_ranges(
        operand: std::cmp::Ordering,
        cutoff: u64,
        range: RangeInclusive<u64>,
    ) -> Result<(RangeInclusive<u64>, RangeInclusive<u64>), String> {
        match operand {
            std::cmp::Ordering::Less => {
                let negative_range = cutoff..=*range.end();
                #[allow(clippy::range_minus_one)]
                let positive_range = *range.start()..=(cutoff - 1);
                Ok((negative_range, positive_range))
            }
            std::cmp::Ordering::Greater => {
                let positive_range = (cutoff + 1)..=*range.end();
                let negative_range = *range.start()..=cutoff;
                Ok((negative_range, positive_range))
            }
            std::cmp::Ordering::Equal => Err("Rule had unusable operand.".to_owned()),
        }
    }
}

impl<'a> TryFrom<&'a str> for Workflow<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let is_comparator = |c| c == '<' || c == '>';

        let (name, rules_str) = {
            if let Some((name, rest)) = value.split_once('{') {
                (name, rest.trim_end_matches('}'))
            } else {
                Err("Malformed workflow.")?
            }
        };
        let rules = rules_str
            .split(',')
            .map(|rule_str| {
                if let Some((condition, next)) = rule_str.split_once(':') {
                    match condition
                        .split_inclusive(is_comparator)
                        .collect::<Vec<_>>()
                        .as_slice()
                    {
                        [rule_operand, value] => {
                            let (rating_type, operand) =
                                { rule_operand.split_at(rule_operand.len() - 1) };
                            let value = value
                                .parse()
                                .map_err(|err| format!("Failed to parse rating value. '{err}'"))?;
                            let rating = match rating_type {
                                "x" => Rating::Cool(value),
                                "m" => Rating::Musical(value),
                                "a" => Rating::Aerodynamic(value),
                                "s" => Rating::Shiny(value),
                                t => Err(format!("Unknown rating type '{t}'."))?,
                            };
                            let operand = match operand {
                                "<" => std::cmp::Ordering::Less,
                                ">" => std::cmp::Ordering::Greater,
                                o => Err(format!("Unknown operand '{o}'."))?,
                            };
                            Ok(Rule {
                                rating,
                                response: next,
                                operand,
                            })
                        }
                        _ => Err(format!("Malformed rule. '{rule_str}'")),
                    }
                } else {
                    Ok(Rule {
                        rating: Rating::Cool(u64::MAX),
                        response: rule_str,
                        operand: std::cmp::Ordering::Less,
                    })
                }
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(Self { name, rules })
    }
}
