use std::collections::HashMap;

static PROBLEM_NAME: &str = "2020-01";

fn main() {
    advent_of_code::init();

    let input = advent_of_code::load_input(PROBLEM_NAME);
    let nums: Vec<i32> = advent_of_code::utils::decode_line(&input);

    let pair = find_pair(2020, &nums);
    log::info!("Part 1 = {}", pair.0 * pair.1);

    let nums = find_three_pair(2020, &nums);
    log::info!("Part 2 = {}", nums.0 * nums.1 * nums.2);
}

/// Attempt to find a pair that sum to some number.
///
/// Runs in `O(n)` time.
///
/// # Panics
///
/// Will panic if no pair can be found.
fn find_pair(sums_to: i32, nums: &[i32]) -> (i32, i32) {
    log::debug!("Attempting to find two numbers summing to {}", sums_to);

    let mut index = HashMap::new();

    // Iterate through each number. As we go through, calculate what other
    // number would be required to reach our desired total. In each loop,
    // calculate what other number is required. If this number has previously
    // been seen, we've found our pair.
    for num in nums {
        if let Some(other) = index.get(num) {
            log::debug!("Found pair {} + {} = {}", num, other, sums_to);
            return (*num, *other);
        }

        let remainder = sums_to - num;
        log::trace!("Input {} has remainder {}", num, remainder);

        index.insert(remainder, *num);
    }

    panic!("unable to find pair");
}

/// Attempt to find three numbers that sum to some number.
///
/// Runs in `O(n^3)` time. There are accessible `O(n^2)` to visit in the future.
///
/// # Panics
///
/// Will panic if no numbers can be found.
fn find_three_pair(sums_to: i32, nums: &[i32]) -> (i32, i32, i32) {
    log::debug!("Attempting to find three numbers summing to {}", sums_to);

    let mut checks = 0;

    // A nested for loop to brute force each possible combination.
    //
    // Not for use in production code or with untrusted input :)
    for (idx1, n1) in nums.iter().enumerate() {
        for (idx2, n2) in nums.iter().enumerate() {
            // We can't reuse numbers, so filter out any items where we're
            // looking at the same line.
            if idx1 == idx2 {
                continue
            }

            for (idx3, n3) in nums.iter().enumerate() {
                if idx2 == idx3 {
                    continue
                }

                let sum = n1 + n2 + n3;
                log::trace!("Looking at combination {} + {} + {} = {}", n1, n2, n3, sum);

                checks += 1;

                if sum == sums_to {
                    log::debug!(
                        "Found three pair {} + {} + {} = {} with {} checks",
                        n1,
                        n2,
                        n3,
                        sums_to,
                        checks
                    );
                    log::trace!("Pairs had positions {}, {}, {}", idx1, idx2, idx3);

                    return (*n1, *n2, *n3);
                }
            }
        }
    }

    panic!("unable to find numbers");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_pair() {
        advent_of_code::init();

        let pair = find_pair(5, &[1, 2, 3, 4, 5]);
        assert_eq!(pair, (3, 2));
    }

    #[test]
    fn test_find_three_pair() {
        advent_of_code::init();

        let pair = find_three_pair(6, &[1, 2, 3, 4, 5]);
        assert_eq!(pair, (1, 2, 3));
    }
}
