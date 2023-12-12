use super::Loop;

pub fn to_ppm(pipes: &[&[u8]], line_length: usize, map: &[Loop]) -> std::io::Result<()> {
    use std::io::Write;

    let mut file = std::fs::File::create("test.ppm")?;
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", pipes[0].len() * 3, pipes.len() * 3)?;
    writeln!(file, "255")?;
    writeln!(file)?;
    let zipper = pipes.iter().copied().flatten().zip(map).collect::<Vec<_>>();
    for chunk in zipper.chunks(line_length) {
        // top row
        for tile in chunk {
            match tile {
                (_, Loop::Shadowed) => {
                    write!(file, "0 255 0 0 255 0 0 255 0 ")?;
                }
                (b'S', _) => {
                    write!(file, "0 0 255 0 0 255 0 0 255 ")?;
                }
                (b'.', Loop::Outside) => {
                    write!(file, "255 0 0 255 0 0 255 0 0 ")?;
                }
                (b'-', Loop::Outside) => {
                    write!(file, "255 0 0 255 0 0 255 0 0 ")?;
                }
                (b'|', Loop::Outside) => {
                    write!(file, "255 0 0 0 0 0 255 0 0 ")?;
                }
                (b'F', Loop::Outside) => {
                    write!(file, "255 0 0 255 0 0 255 0 0 ")?;
                }
                (b'7', Loop::Outside) => {
                    write!(file, "255 0 0 255 0 0 255 0 0 ")?;
                }
                (b'J', Loop::Outside) => {
                    write!(file, "255 0 0 0 0 0 255 0 0 ")?;
                }
                (b'L', Loop::Outside) => {
                    write!(file, "255 0 0 0 0 0 255 0 0 ")?;
                }
                (b'-', Loop::StraightPipeUpOutside) => {
                    write!(file, "255 0 0 255 0 0 255 0 0 ")?;
                }
                (b'|', Loop::StraightPipeLeftOutside) => {
                    write!(file, "255 0 0 0 0 0 0 128 0 ")?;
                }
                (b'F', Loop::NWPipeOuter) => {
                    write!(file, "255 0 0 255 0 0 255 0 0 ")?;
                }
                (b'7', Loop::NEPipeOuter) => {
                    write!(file, "255 0 0 255 0 0 255 0 0 ")?;
                }
                (b'J', Loop::SEPipeOuter) => {
                    write!(file, "0 128 0 0 0 0 255 0 0 ")?;
                }
                (b'L', Loop::SWPipeOuter) => {
                    write!(file, "255 0 0 0 0 0 0 128 0 ")?;
                }
                (b'-', Loop::StraightPipeDownOutside) => {
                    write!(file, "0 128 0 0 128 0 0 128 0 ")?;
                }
                (b'|', Loop::StraightPipeRightOutside) => {
                    write!(file, "0 128 0 0 0 0 255 0 0 ")?;
                }
                (b'F', Loop::NWPipeInner) => {
                    write!(file, "0 128 0 0 128 0 0 128 0 ")?;
                }
                (b'7', Loop::NEPipeInner) => {
                    write!(file, "0 128 0 0 128 0 0 128 0 ")?;
                }
                (b'J', Loop::SEPipeInner) => {
                    write!(file, "255 0 0 0 0 0 0 128 0 ")?;
                }
                (b'L', Loop::SWPipeInner) => {
                    write!(file, "0 128 0 0 0 0 255 0 0 ")?;
                }
                _ => {
                    write!(file, "255 0 255 255 0 255 255 0 255 ")?;
                }
            }
        }
        writeln!(file)?;

        // mid row
        for tile in chunk {
            match tile {
                (_, Loop::Shadowed) => {
                    write!(file, "0 255 0 0 255 0 0 255 0 ")?;
                }
                (b'S', _) => {
                    write!(file, "0 0 255 0 0 255 0 0 255 ")?;
                }
                (b'.', Loop::Outside) => {
                    write!(file, "255 0 0 255 0 0 255 0 0 ")?;
                }
                (b'-', Loop::Outside) => {
                    write!(file, "0 0 0 0 0 0 0 0 0 ")?;
                }
                (b'-', Loop::StraightPipeUpOutside) => {
                    write!(file, "0 0 0 0 0 0 0 0 0 ")?;
                }
                (b'-', Loop::StraightPipeDownOutside) => {
                    write!(file, "0 0 0 0 0 0 0 0 0 ")?;
                }
                (b'|', Loop::Outside) => {
                    write!(file, "255 0 0 0 0 0 255 0 0 ")?;
                }
                (b'|', Loop::StraightPipeLeftOutside) => {
                    write!(file, "255 0 0 0 0 0 0 128 0 ")?;
                }
                (b'|', Loop::StraightPipeRightOutside) => {
                    write!(file, "0 128 0 0 0 0 255 0 0 ")?;
                }
                (b'F', Loop::Outside) => {
                    write!(file, "255 0 0 0 0 0 0 0 0 ")?;
                }
                (b'F', Loop::NWPipeOuter) => {
                    write!(file, "255 0 0 0 0 0 0 0 0 ")?;
                }
                (b'F', Loop::NWPipeInner) => {
                    write!(file, "0 128 0 0 0 0 0 0 0 ")?;
                }
                (b'7', Loop::Outside) => {
                    write!(file, "0 0 0 0 0 0 255 0 0 ")?;
                }
                (b'7', Loop::NEPipeOuter) => {
                    write!(file, "0 0 0 0 0 0 255 0 0 ")?;
                }
                (b'7', Loop::NEPipeInner) => {
                    write!(file, "0 0 0 0 0 0 0 128 0 ")?;
                }
                (b'J', Loop::Outside) => {
                    write!(file, "0 0 0 0 0 0 255 0 0 ")?;
                }
                (b'J', Loop::SEPipeOuter) => {
                    write!(file, "0 0 0 0 0 0 255 0 0 ")?;
                }
                (b'J', Loop::SEPipeInner) => {
                    write!(file, "0 0 0 0 0 0 0 128 0 ")?;
                }
                (b'L', Loop::Outside) => {
                    write!(file, "255 0 0 0 0 0 0 0 0 ")?;
                }
                (b'L', Loop::SWPipeOuter) => {
                    write!(file, "255 0 0 0 0 0 0 0 0 ")?;
                }
                (b'L', Loop::SWPipeInner) => {
                    write!(file, "0 128 0 0 0 0 0 0 0 ")?;
                }
                _ => {
                    write!(file, "255 0 255 255 0 255 255 0 255 ")?;
                }
            }
        }
        writeln!(file)?;

        // bottom row
        for tile in chunk {
            match tile {
                (_, Loop::Shadowed) => {
                    write!(file, "0 255 0 0 255 0 0 255 0 ")?;
                }
                (b'S', _) => {
                    write!(file, "0 0 255 0 0 255 0 0 255 ")?;
                }
                (b'.', Loop::Outside) => {
                    write!(file, "255 0 0 255 0 0 255 0 0 ")?;
                }
                (b'-', Loop::Outside) => {
                    write!(file, "255 0 0 255 0 0 255 0 0 ")?;
                }
                (b'-', Loop::StraightPipeUpOutside) => {
                    write!(file, "0 128 0 0 128 0 0 128 0 ")?;
                }
                (b'-', Loop::StraightPipeDownOutside) => {
                    write!(file, "255 0 0 255 0 0 255 0 0 ")?;
                }
                (b'|', Loop::Outside) => {
                    write!(file, "255 0 0 0 0 0 255 0 0 ")?;
                }
                (b'|', Loop::StraightPipeLeftOutside) => {
                    write!(file, "255 0 0 0 0 0 0 128 0 ")?;
                }
                (b'|', Loop::StraightPipeRightOutside) => {
                    write!(file, "0 128 0 0 0 0 255 0 0 ")?;
                }
                (b'F', Loop::Outside) => {
                    write!(file, "255 0 0 0 0 0 255 0 0 ")?;
                }
                (b'F', Loop::NWPipeOuter) => {
                    write!(file, "255 0 0 0 0 0 0 128 0 ")?;
                }
                (b'F', Loop::NWPipeInner) => {
                    write!(file, "0 128 0 0 0 0 255 0 0 ")?;
                }
                (b'7', Loop::Outside) => {
                    write!(file, "255 0 0 0 0 0 255 0 0 ")?;
                }
                (b'7', Loop::NEPipeOuter) => {
                    write!(file, "0 128 0 0 0 0 255 0 0 ")?;
                }
                (b'7', Loop::NEPipeInner) => {
                    write!(file, "255 0 0 0 0 0 0 128 0 ")?;
                }
                (b'J', Loop::Outside) => {
                    write!(file, "255 0 0 255 0 0 255 0 0 ")?;
                }
                (b'J', Loop::SEPipeOuter) => {
                    write!(file, "255 0 0 255 0 0 255 0 0 ")?;
                }
                (b'J', Loop::SEPipeInner) => {
                    write!(file, "0 128 0 0 128 0 0 128 0 ")?;
                }
                (b'L', Loop::Outside) => {
                    write!(file, "255 0 0 255 0 0 255 0 0 ")?;
                }
                (b'L', Loop::SWPipeOuter) => {
                    write!(file, "255 0 0 255 0 0 255 0 0 ")?;
                }
                (b'L', Loop::SWPipeInner) => {
                    write!(file, "0 128 0 0 128 0 0 128 0 ")?;
                }
                _ => {
                    write!(file, "255 0 255 255 0 255 255 0 255 ")?;
                }
            }
            writeln!(file)?;
        }
    }

    Ok(())
}
