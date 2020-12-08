const PROBLEM_NAME: &str = "2015-01";

fn main() {
    advent_of_code::init();

    let input = advent_of_code::load_input(PROBLEM_NAME);

    let floors = process_floors(&input);
    log::info!("Part 1 = {}", floors);

    let stops = stops_before_floor(&input, -1);
    log::info!("Part 2 = {}", stops);
}

/// Decode a parenthesis into ascending (+1) or decending (-1).
///
/// # Panics
///
/// Will cause a panic if the passed character is not a parenthesis.
fn decode_paren(paren: char) -> i32 {
    match paren {
        '(' => 1,
        ')' => -1,
        _ => panic!("unexpected symbol"),
    }
}

/// Iterate through input to sum floors.
fn process_floors(input: &str) -> i32 {
    input.chars().into_iter().map(decode_paren).sum()
}

/// Calculate the number of parentheses before reaching a desired floor.
///
/// # Panics
///
/// Will panic if the floor is never reached.
fn stops_before_floor(input: &str, desired: i32) -> i32 {
    let mut floor = 0;

    // Iterate through each parenthesis, taking note of the position.
    for (position, paren) in input.chars().into_iter().enumerate() {
        floor += decode_paren(paren);

        log::trace!("Position {} moved to floor {}", position, floor);

        // If we reach the desired floor, return the position. We need the
        // position as a 1-indexed value rather than 0-indexed, so add 1.
        if floor == desired {
            return position as i32 + 1;
        }
    }

    panic!("never reached floor");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_paren() {
        assert_eq!(decode_paren('('), 1);
        assert_eq!(decode_paren(')'), -1);
    }

    #[test]
    fn test_process_floors() {
        let floors = process_floors("((()");
        assert_eq!(floors, 2);

        let floors = process_floors("))");
        assert_eq!(floors, -2);
    }

    #[test]
    fn test_stops_before_floor() {
        let stops = stops_before_floor(")", -1);
        assert_eq!(stops, 1);

        let stops = stops_before_floor("()())", -1);
        assert_eq!(stops, 5);

        let stops = stops_before_floor("()()(()", 2);
        assert_eq!(stops, 6);
    }
}
