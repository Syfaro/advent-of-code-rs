use std::collections::HashMap;

use itertools::Itertools;

const PROBLEM_NAME: &str = "2015-05";

static VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];
static BLOCKLISTED: &[&str] = &["ab", "cd", "pq", "xy"];

fn main() {
    advent_of_code::init();

    let input = advent_of_code::load_input(PROBLEM_NAME);
    let lines: Vec<String> = advent_of_code::utils::decode_line(&input);

    let nice = lines.iter().filter(|line| is_nice_1(line)).count();
    log::info!("Part 1 = {}", nice);

    let nice = lines.iter().filter(|line| is_nice_2(line)).count();
    log::info!("Part 2 = {}", nice);
}

/// Determine if an input is nice by ensuring it has at least 3 vowels, contains
/// no blocklisted words, and contains repeating characters.
fn is_nice_1(line: &str) -> bool {
    contains_n_vowels(&line, 3) && !blocklisted(&line) && contains_repeating(&line, 1)
}

/// Determine if an input is nice by ensuring it has two non-overlapping pairs
/// and contains a letter that is repeated after a different letter.
fn is_nice_2(line: &str) -> bool {
    pairs(&line) && contains_repeating(&line, 2)
}

/// Determine if an input contains at least n number of vowels.
fn contains_n_vowels(input: &str, n: usize) -> bool {
    input.chars().filter(|c| VOWELS.contains(c)).count() >= n
}

/// Determine if the input contains blocklisted characters.
fn blocklisted(input: &str) -> bool {
    BLOCKLISTED.iter().any(|letters| input.contains(letters))
}

/// Determine if the input has characters that repeat n characters above it.
fn contains_repeating(input: &str, above: usize) -> bool {
    input.chars().enumerate().any(|(index, c)| {
        if let Some(next) = input.chars().nth(index + above) {
            c == next
        } else {
            false
        }
    })
}

/// Determine if there is a non-overlapping pair of the same character.
fn pairs(input: &str) -> bool {
    // Store the positions of each possible character combination.
    let mut pairs: HashMap<&str, Vec<(usize, usize)>> = HashMap::new();

    // Iterate through the input, collecting every 2-character combination and
    // recording the position.
    for index in 0..input.len() - 1 {
        let pair = &input[index..=index + 1];
        let entry = pairs.entry(pair).or_default();
        entry.push((index, index + 1));
    }

    log::trace!("Calculated pairs {:?}", pairs);

    // Go through each pair's values to determine if there were two
    // non-overlapping occurances of the pair.
    for pair in pairs.values() {
        if pair.len() == 1 {
            continue;
        }

        if pair.len() > 1 {
            log::trace!("Found possible valid pair: {:?}", pair);

            // We need to perform a cartesian product to ensure that all
            // possible combinations of the ranges are tried together. Two
            // characters can have overlapping characters in one instance but
            // still have a valid instance later.
            for (pos1, pos2) in pair.iter().cartesian_product(pair.iter()) {
                log::trace!("Evaluating combination {:?}, {:?}", pos1, pos2);

                if pos1.1 < pos2.0 {
                    log::trace!("Found valid pair: {:?}", pair);
                    return true;
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_n_vowels() {
        advent_of_code::init();

        let input = "aeiou";
        assert_eq!(contains_n_vowels(&input, 3), true);

        let input = "bcdfg";
        assert_eq!(contains_n_vowels(&input, 3), false);

        let input = "ugknbfddgicrmopn";
        assert_eq!(contains_n_vowels(&input, 3), true);
    }

    #[test]
    fn test_blocklisted() {
        advent_of_code::init();

        let input = "abcdefg";
        assert_eq!(blocklisted(&input), true);

        let input = "asdf";
        assert_eq!(blocklisted(&input), false);

        let input = "ugknbfddgicrmopn";
        assert_eq!(blocklisted(&input), false);
    }

    #[test]
    fn test_contains_repeating() {
        advent_of_code::init();

        let input = "abcdefg";
        assert_eq!(contains_repeating(&input, 1), false);

        let input = "abbb";
        assert_eq!(contains_repeating(&input, 1), true);

        let input = "ugknbfddgicrmopn";
        assert_eq!(contains_repeating(&input, 1), true);

        let input = "abcdefeghi";
        assert_eq!(contains_repeating(&input, 2), true);
    }

    #[test]
    fn test_pairs() {
        advent_of_code::init();

        let input = pairs("aabcdefgaa");
        assert_eq!(input, true);

        let input = pairs("aaa");
        assert_eq!(input, false);
    }

    #[test]
    fn test_is_nice_1() {
        advent_of_code::init();

        let inputs = &[
            ("ugknbfddgicrmopn", true),
            ("aaa", true),
            ("jchzalrnumimnmhp", false),
            ("haegwjzuvuyypxyu", false),
            ("dvszwmarrgswjxmb", false),
        ];
        for (input, nice) in inputs {
            let is_nice = is_nice_1(&input);
            assert_eq!(is_nice, *nice, "input {} is {}", input, nice);
        }
    }

    #[test]
    fn test_is_nice_2() {
        advent_of_code::init();

        let inputs = &[
            ("qjhvhtzxzqqjkmpb", true),
            ("xxyxx", true),
            ("uurcxstgmygtbstg", false),
            ("ieodomkazucvgmuy", false),
        ];
        for (input, nice) in inputs {
            let is_nice = is_nice_2(&input);
            assert_eq!(is_nice, *nice, "input {} is {}", input, nice);
        }
    }
}
