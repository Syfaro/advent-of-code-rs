use std::collections::HashMap;

static PROBLEM_NAME: &str = "2020-10";

fn main() {
    advent_of_code::init();

    let input = advent_of_code::load_input(PROBLEM_NAME);
    let mut lines: Vec<usize> = advent_of_code::utils::decode_line(&input);
    fix_input(&mut lines);

    let steps = count_steps(&lines);
    log::info!("Part 1 = {}", steps[&1] * steps[&3]);

    let mut visited = Default::default();
    let paths = count_paths(&lines, 0, &mut visited);
    log::info!("Part 2 = {}", paths);
}

/// Clean up input by sorting entries, inserting a zero value at the beginning,
/// and a value 3 greater than the last value.
fn fix_input(jolts: &mut Vec<usize>) {
    jolts.sort();
    jolts.insert(0, 0);
    jolts.push(jolts.last().unwrap() + 3);
}

/// Count the number of each size of step between values. Items must be sorted
/// for this to work.
fn count_steps(jolts: &[usize]) -> HashMap<usize, usize> {
    let mut counts = HashMap::new();

    for index in 0..jolts.len() - 1 {
        let diff = jolts[index + 1] - jolts[index];
        log::trace!(
            "Index {} to index {} had difference of {}",
            index,
            index + 1,
            diff
        );
        let entry = counts.entry(diff).or_default();
        *entry += 1;
    }

    counts
}

/// Count the number of possible paths so that all values are 1 to 3 numbers
/// apart.
fn count_paths(jolts: &[usize], pos: usize, mut visited: &mut HashMap<usize, usize>) -> usize {
    // If we're at the end, there is exactly one path to the end from here. This
    // is the only place an actual number comes from, everything else is
    // counting the number of times this position is reached.
    if pos == jolts.len() - 1 {
        log::trace!("Pos {} is the end", pos);
        return 1;
    }

    // Check if we've already calculated the number of paths from this position.
    // When we're starting from the same point, the number of possibilities is
    // always the same.
    if let Some(count) = visited.get(&pos) {
        log::trace!("Already evaluated from pos {}: {}", pos, count);
        return *count;
    }

    // Calculate the number of paths through children by:
    // * Iterating through each element between the next position to the end.
    //   We can skip the current position because we're already it.
    // * Calculating the difference between us and the next position to check
    //   if the difference is less than 3, making it a valid possibility.
    // * Recursively calculating everything for each child and adding together
    //   all of the possibilities.
    //
    // We only really need to iterate from `pos + 1` to
    // `min(pos + 4, jolts.len())` but we'd still need the diff check, so it
    // would look more complicated and have identical performance.
    let mut paths = 0;
    for index in pos + 1..jolts.len() {
        let diff = jolts[index] - jolts[pos];

        // Because the values are sorted, as soon as we're over a difference of
        // 3 there are no more valid possibilities.
        if diff > 3 {
            log::trace!("No more possibilities from pos {} to index {}", pos, index);
            break;
        }

        log::trace!("Pos {} to index {} has diff of {}", pos, index, diff);
        paths += count_paths(&jolts, index, &mut visited);
    }

    // Record the value so we don't have to recalculate it in the future.
    visited.insert(pos, paths);

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_input() {
        advent_of_code::init();

        let mut input = vec![4, 232, 65];
        fix_input(&mut input);
        assert_eq!(input, vec![0, 4, 65, 232, 235]);
    }

    #[test]
    fn test_count_steps() {
        advent_of_code::init();

        let input = &[0, 1, 4, 7, 8, 9];
        let steps = count_steps(input);
        assert_eq!(steps.len(), 2);
        assert_eq!(steps[&1], 3);
        assert_eq!(steps[&3], 2);

        let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        fix_input(&mut input);
        let steps = count_steps(&input);
        assert_eq!(steps.len(), 2);
        assert_eq!(steps[&1], 7);
        assert_eq!(steps[&3], 5);
    }

    #[test]
    fn test_count_paths() {
        advent_of_code::init();

        // Only one path can happen here, directly from start to end.
        let input = &[0, 1];
        let mut visited = Default::default();
        let paths = count_paths(input, 0, &mut visited);
        assert_eq!(paths, 1);

        // 4 paths can happen here:
        // * 0 --> 1 --> 2 --> 3
        //      1     1     1
        //
        // * 0 --> 1 --> 3
        //      1     2
        //
        // * 0 --> 2 --> 3
        //      2     1
        //
        // * 0 --> 3
        //      3
        let input = &[0, 1, 2, 3];
        let mut visited = Default::default();
        let paths = count_paths(input, 0, &mut visited);
        assert_eq!(paths, 4);
    }
}
