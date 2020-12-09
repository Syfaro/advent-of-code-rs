use std::collections::VecDeque;

static PROBLEM_NAME: &str = "2020-09";

fn main() {
    advent_of_code::init();

    let input = advent_of_code::load_input(PROBLEM_NAME);
    let numbers: Vec<i64> = advent_of_code::utils::decode_line(&input);

    let first_invalid = find_first_invalid(&numbers, 25);
    log::info!("Part 1 = {}", first_invalid);

    let numbers = find_any_that_sum(&numbers, first_invalid);
    let sum = sum_min_max(&numbers);
    log::info!("Part 2 = {}", sum);
}

/// Find the first number which cannot be created by summing any two of the
/// previous n values.
///
/// # Panics
///
/// Will panic if there are no numbers that solve it.
fn find_first_invalid(lines: &[i64], previous_count: usize) -> i64 {
    // Allocate a buffer for our n previous values, plus an extra for when a
    // new value has been added.
    let mut previous = VecDeque::with_capacity(previous_count + 1);

    for line in lines {
        // Make sure only the correct number of values are being evaluated.
        while previous.len() > previous_count {
            log::trace!(
                "Removing old previous value, length is {} while desired is {}",
                previous.len(),
                previous_count
            );

            previous.pop_front();
        }

        // Only start evaluating once we have processed n previous values.
        if previous.len() >= previous_count {
            log::trace!("Evaluating {} with previous values {:?}", line, previous);
            let mut has_sum = false;

            // Iterate through each combination, checking if any value sums to
            // the line. Once we've found a single combination, we can break
            // early.
            'outer: for (i1, p1) in previous.iter().enumerate() {
                for (i2, p2) in previous.iter().enumerate() {
                    if i1 == i2 {
                        continue;
                    }

                    if p1 + p2 == *line {
                        log::trace!("Combination {} + {} = {}", p1, p2, line);
                        has_sum = true;
                        break 'outer;
                    }
                }
            }

            // If we never found the numbers, we've found our solution.
            if !has_sum {
                log::debug!("{} has no previous combinations", line);
                return *line;
            }
        }

        previous.push_back(*line);
    }

    panic!("unable to find invalid entry");
}

/// Find two or more numbers that sum to a total.
///
/// # Panics
///
/// This will probably panic if there are no working combinations.
fn find_any_that_sum(lines: &[i64], sum_to: i64) -> &[i64] {
    // Start by looking at at least 2 numbers
    let mut nums = 2;

    loop {
        log::debug!("Checking combinations with {} numbers", nums);

        // Iterate through each possible number, minus the number we're trying.
        // This strategy is one way of making sure we don't exceed the length
        // when trying larger number combinations. If any of these combinations
        // sum to our desired value, we're done.
        for pos in nums..lines.len() {
            let check = &lines[pos - nums..pos];
            if check.iter().sum::<i64>() == sum_to {
                log::debug!("Found combination {:?} that sums to {}", check, sum_to);
                return check;
            }
        }

        nums += 1;
    }
}

/// Sum the min and max numbers.
///
/// # Panics
///
/// Will panic if there is not at least one element provided.
fn sum_min_max(numbers: &[i64]) -> i64 {
    numbers.iter().min().unwrap() + numbers.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static NUMBERS: &[i64] = &[
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn test_find_first_invalid() {
        advent_of_code::init();

        let first_invalid = find_first_invalid(NUMBERS, 5);
        assert_eq!(first_invalid, 127);
    }

    #[test]
    fn test_find_any_that_sum() {
        advent_of_code::init();

        let numbers = find_any_that_sum(NUMBERS, 127);
        assert_eq!(numbers, &[15, 25, 47, 40]);
    }

    #[test]
    fn test_sum_min_max() {
        let numbers = &[1, 2, 3];
        assert_eq!(sum_min_max(numbers), 4);
    }
}
