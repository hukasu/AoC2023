mod graph;

use std::{io::Read, num::TryFromIntError};

use crate::graph::Graph;

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day25_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = split_network(&input, 3);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    // let timer = std::time::Instant::now();
                    print_tree();
                    // println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn split_network(input: &str, target_cuts: usize) -> Result<u64, String> {
    let graph: Graph = input.try_into()?;
    let (l_bucket, r_bucket) = graph.partition(target_cuts)?;
    (l_bucket.len() * r_bucket.len())
        .try_into()
        .map_err(|err: TryFromIntError| err.to_string())
}

fn print_tree() {
    // Source https://www.asciiart.eu/holiday-and-events/christmas/trees#google_vignette
    static TREE: &str = r"         |
        -+-
         A
        /=\               /\  /\    ___  _ __  _ __ __    __
      i/ O \i            /  \/  \  / _ \| '__|| '__|\ \  / /
      /=====\           / /\  /\ \|  __/| |   | |    \ \/ /
      /  i  \           \ \ \/ / / \___/|_|   |_|     \  /
    i/ O * O \i                                       / /
    /=========\        __  __                        /_/    _
    /  *   *  \        \ \/ /        /\  /\    __ _  ____  | |
  i/ O   i   O \i       \  /   __   /  \/  \  / _` |/ ___\ |_|
  /=============\       /  \  |__| / /\  /\ \| (_| |\___ \  _
  /  O   i   O  \      /_/\_\      \ \ \/ / / \__,_|\____/ |_|
i/ *   O   O   * \i
/=================\
       |___|";
    println!("{TREE}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_INPUT: &str = r"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn part1_test() {
        assert_eq!(split_network(PART1_INPUT, 3), Ok(54));
    }
}
