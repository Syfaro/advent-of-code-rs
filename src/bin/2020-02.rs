use lazy_static::lazy_static;
use regex::Regex;

static PROBLEM_NAME: &str = "2020-02";

lazy_static! {
    /// Regex to match a password file entry, which follows the format:
    ///
    /// ```text
    /// 1-3 a: aaaaa
    /// ```
    static ref PASSWORD_LINE: Regex = Regex::new(r#"(?P<num1>\d+)-(?P<num2>\d+) (?P<letter>\w): (?P<password>\w+)"#).unwrap();
}

/// A password rule and the corresponding password.
#[derive(Clone, Debug, PartialEq)]
struct PasswordEntry {
    num1: usize,
    num2: usize,
    letter: char,
    password: String,
}

impl std::str::FromStr for PasswordEntry {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = PASSWORD_LINE.captures(&s).ok_or("unable to match regex")?;

        Ok(PasswordEntry {
            // These should always be numbers.
            num1: captures["num1"]
                .parse()
                .map_err(|_| "num1 was not number")?,
            num2: captures["num2"]
                .parse()
                .map_err(|_| "num2 was not number")?,
            // We always should have a character here.
            letter: captures["letter"].chars().next().ok_or("missing letter")?,
            password: captures["password"].to_string(),
        })
    }
}

fn main() {
    advent_of_code::init();

    let input = advent_of_code::load_input(PROBLEM_NAME);
    let entries: Vec<PasswordEntry> = advent_of_code::utils::decode_line(&input);

    let valid_passwords = count_valid_letter_counts(&entries);
    log::info!("Part 1 = {}", valid_passwords);

    let valid_passwords = count_valid_letter_positions(&entries);
    log::info!("Part 2 = {}", valid_passwords);
}

/// Validate that the letter in the entry occurs (num1, num2) times.
fn validate_letter_count(entry: &PasswordEntry) -> bool {
    let count = entry.password.matches(entry.letter).count();

    log::trace!(
        "Password {} has letter {} appear {} times",
        entry.password,
        entry.letter,
        count
    );

    entry.num1 <= count && count <= entry.num2
}

/// Validate that the letter appears in num1 XOR num2.
fn validate_letter_position(entry: &PasswordEntry) -> bool {
    // We need to subtract one from these because rules are 1-indexed.
    let pos1 = entry.password.chars().nth(entry.num1 - 1).unwrap() == entry.letter;
    let pos2 = entry.password.chars().nth(entry.num2 - 1).unwrap() == entry.letter;

    log::trace!(
        "Password {} has letter {} in pos1: {}, pos2: {}",
        entry.password,
        entry.letter,
        pos1,
        pos2
    );

    pos1 ^ pos2
}

/// Count the number of true values returned by [`validate_letter_count`].
fn count_valid_letter_counts(entries: &[PasswordEntry]) -> usize {
    entries
        .iter()
        .filter(|entry| validate_letter_count(entry))
        .count()
}

/// Count the number of true values returned by [`validate_letter_position`].
fn count_valid_letter_positions(entries: &[PasswordEntry]) -> usize {
    entries
        .iter()
        .filter(|entry| validate_letter_position(entry))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{validate_letter_count, validate_letter_position, PasswordEntry};

    #[test]
    fn test_password_entry_from_str() {
        advent_of_code::init();

        let line = "3-12 v: zbvlbpxcrnvvwjpwl";
        let password_entry: PasswordEntry = line.parse().unwrap();
        assert_eq!(
            password_entry,
            PasswordEntry {
                num1: 3,
                num2: 12,
                letter: 'v',
                password: "zbvlbpxcrnvvwjpwl".to_string(),
            }
        );
    }

    #[test]
    fn test_validate_letter_count() {
        advent_of_code::init();

        let entry: PasswordEntry = "3-12 v: vvvvvvvvv".parse().unwrap();
        let validation = validate_letter_count(&entry);
        assert_eq!(validation, true);

        let entry: PasswordEntry = "3-12 v: zbvlbpxcrnvvwjpwl".parse().unwrap();
        let validation = validate_letter_count(&entry);
        assert_eq!(validation, false);
    }

    #[test]
    fn test_validate_letter_position() {
        advent_of_code::init();

        let entry: PasswordEntry = "3-12 v: xxvxxxxxxxxxxxxx".parse().unwrap();
        let validation = validate_letter_position(&entry);
        assert_eq!(validation, true);
    }
}
