/// Decode each line from an input into a type.
///
/// # Panics
///
/// This will panic if a line cannot be parsed.
pub fn decode_line<T: std::str::FromStr>(input: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|line| {
            line.parse()
                .expect(&format!("{} cannot be parsed into desired type", line))
        })
        .collect()
}
