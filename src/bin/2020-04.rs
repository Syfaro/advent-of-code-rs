use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use lazy_static::lazy_static;

static PROBLEM_NAME: &str = "2020-04";

lazy_static! {
    /// All of the fields required for each passport entry.
    static ref REQUIRED_FIELDS: HashSet<&'static str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"].into_iter().collect();
    /// All known eye colors.
    static ref EYE_COLORS: HashSet<&'static str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].into_iter().collect();
}

fn main() {
    advent_of_code::init();

    let input = advent_of_code::load_input(PROBLEM_NAME);
    let passports = parse_passports(&input);
    log::debug!("Parsed {} passports", passports.len());

    let valid_passports: u32 = passports
        .iter()
        .map(validate_fields)
        .map(|valid| valid as u32)
        .sum();
    log::info!("Part 1 = {}", valid_passports);

    let valid_passports: u32 = passports
        .iter()
        .map(|passport| validate_fields(&passport) && validate_contents(&passport))
        .map(|valid| valid as u32)
        .sum();
    log::info!("Part 2 = {}", valid_passports);
}

/// Validate a number by ensuring it is between a minimum and maximum value.
fn validate_number(number: u32, minimum: u32, maximum: u32) -> bool {
    number >= minimum && number <= maximum
}

/// Parse passports into a vec of key value pairs.
fn parse_passports(input: &str) -> Vec<HashMap<&str, &str>> {
    // Storage for all of our parsed passports
    let mut passports = Vec::new();

    // Currently active passport entry
    let mut passport = HashMap::new();

    // Iterate through each line. If the line is empty the entry is complete
    // and needs to be saved. Otherwise, continue adding to existing passport
    // entry.
    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            log::trace!("Parsed new passport: {:?}", passport);

            passports.push(passport.clone());
            passport = HashMap::new();
            continue;
        }

        let fields = line.split(' ');
        for field in fields {
            if field.is_empty() {
                continue;
            }

            let mut parts = field.split(':');
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();

            passport.insert(key, value);
        }
    }

    log::trace!("Parsed new passport: {:?}", passport);
    passports.push(passport);

    passports
}

/// Determine if the passport contains all of the required fields.
fn validate_fields(passport: &HashMap<&str, &str>) -> bool {
    let fields: HashSet<&str> = HashSet::from_iter(passport.keys().into_iter().cloned());
    let mut difference: HashSet<&str> = fields
        .symmetric_difference(&REQUIRED_FIELDS)
        .into_iter()
        .cloned()
        .collect();
    difference.remove("cid");

    log::trace!("Difference between actual and required: {:?}", difference);

    difference.is_empty()
}

/// Check if a character is a valid hexadecimal character.
fn is_hex(c: char) -> bool {
    match c {
        'a'..='z' => true,
        '0'..='9' => true,
        _ => false,
    }
}

/// Determine if all the fields within the passport contain valid data.
fn validate_contents(passport: &HashMap<&str, &str>) -> bool {
    for (key, value) in passport {
        let valid = match *key {
            "byr" => match value.parse() {
                Ok(year) => validate_number(year, 1920, 2002),
                _ => false,
            },
            "iyr" => match value.parse() {
                Ok(year) => validate_number(year, 2010, 2020),
                _ => false,
            },
            "eyr" => match value.parse() {
                Ok(year) => validate_number(year, 2020, 2030),
                _ => false,
            },
            "hgt" => {
                let len = value.len();

                if let Ok(num) = value[..len - 2].parse() {
                    let units = &value[len - 2..];

                    match units {
                        "cm" => validate_number(num, 150, 193),
                        "in" => validate_number(num, 59, 76),
                        _ => false,
                    }
                } else {
                    false
                }
            }
            "hcl" => {
                if value.len() == 7 {
                    value.chars().skip(1).all(is_hex)
                } else {
                    false
                }
            }
            "ecl" => EYE_COLORS.contains(value),
            "pid" => {
                if value.len() == 9 {
                    value.parse::<u32>().is_ok()
                } else {
                    false
                }
            }
            "cid" => true,
            _ => unreachable!("all fields should have been validated"),
        };

        log::trace!("Field {} with data {} validity: {}", key, value, valid);

        if !valid {
            log::debug!(
                "Passport was not valid because field {} contained data {}",
                key,
                value
            );
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    static INVALID_PASSPORT: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926";

    static VALID_PASSPORT: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f";

    #[test]
    fn test_parse_passports() {
        advent_of_code::init();

        let passports = parse_passports(TEST_INPUT);
        assert_eq!(passports.len(), 4);
    }

    #[test]
    fn test_is_hex() {
        advent_of_code::init();

        assert_eq!(is_hex('a'), true);
        assert_eq!(is_hex('.'), false);
    }

    #[test]
    fn test_validate_fields() {
        advent_of_code::init();

        let passports = parse_passports(TEST_INPUT);

        assert_eq!(validate_fields(&passports[0]), true);
        assert_eq!(validate_fields(&passports[1]), false);
    }

    #[test]
    fn test_validate_contents() {
        advent_of_code::init();

        let passports = parse_passports(VALID_PASSPORT);
        assert_eq!(validate_contents(&passports[0]), true);

        let passports = parse_passports(INVALID_PASSPORT);
        assert_eq!(validate_contents(&passports[0]), false);
    }
}
