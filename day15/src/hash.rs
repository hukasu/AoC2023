pub fn hash(input: &str) -> Result<u8, std::num::TryFromIntError> {
    input
        .as_bytes()
        .iter()
        .try_fold(0, |current_value, current_ascii| {
            let mut wide = u16::from(current_value) + u16::from(*current_ascii);
            wide *= 17;
            wide %= 256;
            u8::try_from(wide)
        })
}
