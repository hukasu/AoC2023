use std::collections::{BTreeSet, BinaryHeap};

#[derive(Debug)]
pub struct Graph<'a> {
    pub verteces: Vec<&'a str>,
    pub edges: Vec<(&'a str, &'a str)>,
}

impl<'a> Graph<'a> {
    pub fn partition(
        &self,
        target_cuts: usize,
    ) -> Result<(BTreeSet<&'a str>, BTreeSet<&'a str>), String> {
        let (left_bucket, right_bucket) = {
            let slices = self.verteces.split_at(self.verteces.len() / 2);
            (
                slices.0.iter().copied().collect::<BTreeSet<_>>(),
                slices.1.iter().copied().collect::<BTreeSet<_>>(),
            )
        };
        let (left_bucket, right_bucket) = self.run_partition(left_bucket, right_bucket)?;
        let cuts = self.number_of_cuts(&left_bucket);

        let (left_bucket, right_bucket) = if cuts == target_cuts {
            (left_bucket, right_bucket)
        } else {
            self.run_partition(left_bucket, right_bucket)?
        };
        let cuts = self.number_of_cuts(&left_bucket);

        if cuts == target_cuts {
            Ok((left_bucket, right_bucket))
        } else {
            self.run_partition(left_bucket, right_bucket)
        }
    }

    fn run_partition(
        &self,
        mut left_bucket: BTreeSet<&'a str>,
        mut right_bucket: BTreeSet<&'a str>,
    ) -> Result<(BTreeSet<&'a str>, BTreeSet<&'a str>), String> {
        let mut locked = Vec::new();

        let mut gains = self.calculate_gains(&left_bucket, &right_bucket, &locked);

        let mut cuts = self.number_of_cuts(&left_bucket);
        let mut best_cuts = cuts;

        while let Some(max_gain) = gains.pop() {
            Self::move_vertex(max_gain.1, &mut left_bucket, &mut right_bucket);
            locked.push(max_gain.1);
            gains = self.calculate_gains(&left_bucket, &right_bucket, &locked);
            cuts = self.number_of_cuts(&left_bucket);
            best_cuts = best_cuts.min(cuts);
        }

        while cuts > best_cuts {
            if let Some(locked_vertex) = locked.pop() {
                Self::move_vertex(locked_vertex, &mut left_bucket, &mut right_bucket);
                cuts = self.number_of_cuts(&left_bucket);
            } else {
                return Err("Failed to backtrack.".to_owned());
            }
        }
        Ok((left_bucket, right_bucket))
    }

    fn move_vertex(
        vertex: &'a str,
        left_bucket: &mut BTreeSet<&'a str>,
        right_bucket: &mut BTreeSet<&'a str>,
    ) {
        if left_bucket.contains(vertex) {
            left_bucket.remove(vertex);
            right_bucket.insert(vertex);
        } else {
            right_bucket.remove(vertex);
            left_bucket.insert(vertex);
        }
    }

    fn number_of_cuts(&self, left_bucket: &BTreeSet<&str>) -> usize {
        self.edges
            .iter()
            .filter(|(l, r)| left_bucket.contains(l) != left_bucket.contains(r))
            .count()
    }

    fn calculate_gains(
        &self,
        left_bucket: &BTreeSet<&'a str>,
        right_bucket: &BTreeSet<&'a str>,
        locked: &[&'a str],
    ) -> BinaryHeap<(i64, &'a str)> {
        static EMPTY: BTreeSet<&str> = BTreeSet::new();
        let exclusion = if left_bucket.len().abs_diff(right_bucket.len()) > self.verteces.len() / 5
        {
            match left_bucket.len().cmp(&right_bucket.len()) {
                std::cmp::Ordering::Equal => &EMPTY,
                std::cmp::Ordering::Greater => right_bucket,
                std::cmp::Ordering::Less => left_bucket,
            }
        } else {
            &EMPTY
        };

        self.verteces
            .iter()
            .filter(|vertex| !locked.contains(*vertex) && !exclusion.contains(*vertex))
            .map(|vertex| {
                let in_left_bucket = left_bucket.contains(vertex);
                let (l, r) = self
                    .edges
                    .iter()
                    .filter_map(|edge| {
                        if edge.0 == *vertex {
                            Some(edge.1)
                        } else if edge.1 == *vertex {
                            Some(edge.0)
                        } else {
                            None
                        }
                    })
                    .fold((0, 0), |(l, r), connection| {
                        if left_bucket.contains(&connection) {
                            (l + 1, r)
                        } else {
                            (l, r + 1)
                        }
                    });
                if in_left_bucket {
                    (r - l, *vertex)
                } else {
                    (l - r, *vertex)
                }
            })
            .collect()
    }
}

impl<'a> TryFrom<&'a str> for Graph<'a> {
    type Error = String;
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let mut verteces = vec![];
        let mut edges = vec![];

        for line in input.lines() {
            let (left, right) = line.split_once(':').ok_or("Malformed line.")?;
            if let Err(i) = verteces.binary_search(&left) {
                verteces.insert(i, left);
            };

            right.split_whitespace().for_each(|edge_con| {
                if let Err(i) = verteces.binary_search(&edge_con) {
                    verteces.insert(i, edge_con);
                };
                edges.push((left.min(edge_con), left.max(edge_con)));
            })
        }

        Ok(Self { verteces, edges })
    }
}
