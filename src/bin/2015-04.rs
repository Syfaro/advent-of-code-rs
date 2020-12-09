const PROBLEM_NAME: &str = "2015-04";

fn main() {
    advent_of_code::init();

    let input = advent_of_code::load_input(&PROBLEM_NAME);
    let minimum_number = find_minimum_number(&input, 5);
    log::info!("Part 1 = {}", minimum_number);

    let minimum_number = find_minimum_number(&input, 6);
    log::info!("Part 2 = {}", minimum_number);
}

/// Find the minimum number that when appended to an input creates an md5sum
/// that begins with some number of leading zeros when encoded to a hexadecimal
/// string.
fn find_minimum_number(input: &str, leading_zeros: usize) -> usize {
    // Calculate a string with the desired number of zeros.
    let desired = String::from_utf8(vec![b'0'; leading_zeros]).unwrap();

    let mut num = 0;

    // Loop forever until finding a number that can be appended to the input to
    // create the expected output.
    loop {
        // Compute the md5sum and convert to hex string.
        let hash = md5::compute(format!("{}{}", input, num));
        let h = hex::encode(hash.0);

        log::trace!("Hash with {} was {}", num, h);

        // If they're equal, we've found our result.
        if &h[..leading_zeros] == desired {
            log::debug!("Hash with {} had {} zeros: {}", num, leading_zeros, h);
            break num;
        }

        num += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::find_minimum_number;

    #[test]
    fn test_find_minimum_number() {
        advent_of_code::init();

        let input = "aaaaa";
        assert_eq!(find_minimum_number(&input, 1), 34);
    }
}
