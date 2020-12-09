static PROBLEM_NAME: &str = "2020-05";

const ROW_WIDTH: usize = 8;
const ROWS: usize = 127;
const COLUMNS: usize = 7;

fn main() {
    advent_of_code::init();

    let input = advent_of_code::load_input(PROBLEM_NAME);
    let seats: Vec<String> = advent_of_code::utils::decode_line(&input);

    let highest_seat_id = seats
        .iter()
        .map(|seat| decode_seat(&seat))
        .map(|(row, column)| seat_id(row, column))
        .max()
        .unwrap();
    log::info!("Part 1 = {}", highest_seat_id);

    let missing_seat_id = find_missing_value(&seats);
    log::info!("Part 2 = {}", missing_seat_id);
}

/// Perform a binary space partition from 0 to space. The letters 'B' and 'R' go
/// downwards and all other letters go upwards.
fn binary_space_partition(seat: &str, space: usize) -> usize {
    // In looking up other solutions I realized this could be done by converting
    // the characters to 0s and 1s and get it as a binary representation. I've
    // left it this way as it was my original approach.
    let mut num = 0;
    let mut step = (space + 1) / 2;

    for c in seat.chars() {
        match c {
            'B' | 'R' => num += step,
            _ => (),
        }

        step = step / 2;
    }

    num
}

/// Decode seat letters into a row and column.
fn decode_seat(seat: &str) -> (usize, usize) {
    let row = binary_space_partition(&seat[..7], ROWS);
    let column = binary_space_partition(&seat[7..], COLUMNS);

    (row, column)
}

/// Convert a row and column into a seat ID.
fn seat_id(row: usize, column: usize) -> usize {
    row * ROW_WIDTH + column
}

/// Find an unoccupied seat where the seat is filled before and after it.
fn find_missing_value(seats: &[String]) -> usize {
    let mut seats: Vec<_> = seats
        .iter()
        .map(|seat| decode_seat(&seat))
        .map(|(row, column)| seat_id(row, column))
        .collect();
    seats.sort();

    // Go through each seat, ensure there is a seat above and below it, and if
    // the above and below seats are not exactly 2 apart it means there's a
    // variance which is our empty seat.
    for (index, seat_id) in seats.iter().enumerate() {
        if index > 0 {
            if index < seats.len() - 1 {
                if seats[index + 1] - seats[index - 1] != 2 {
                    // We've found the ID of the seat directly before ours, so
                    // we need to add 1 to get to our unused seat ID.
                    return *seat_id + 1;
                }
            }
        }
    }

    panic!("unable to find missing seat");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_id() {
        advent_of_code::init();

        let id = seat_id(1, 1);
        assert_eq!(id, 9);

        let id = seat_id(8, 7);
        assert_eq!(id, 71);
    }

    #[test]
    fn test_binary_space_partition() {
        advent_of_code::init();

        let seat = binary_space_partition("BFFFBBF", 127);
        assert_eq!(seat, 70);

        let seat = binary_space_partition("RRR", 7);
        assert_eq!(seat, 7);
    }

    #[test]
    fn test_decode_seat() {
        advent_of_code::init();

        let seat = decode_seat("BFFFBBFRRR");
        assert_eq!(seat, (70, 7));
    }

    #[test]
    fn test_find_missing_value() {
        advent_of_code::init();

        let seats = &[
            "RLLRLRRRRL".to_string(),
            "RLLRLRRRRL".to_string(),
            "RLLRRLLLLL".to_string(),
            "RLLRRLLLLR".to_string(),
        ];
        let missing = find_missing_value(seats);
        assert_eq!(missing, 609);
    }
}
