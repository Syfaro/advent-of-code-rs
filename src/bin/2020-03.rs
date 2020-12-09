static PROBLEM_NAME: &str = "2020-03";

const TREE: char = '#';

fn main() {
    advent_of_code::init();

    let input = advent_of_code::load_input(PROBLEM_NAME);
    let lines: Vec<String> = advent_of_code::utils::decode_line(&input);

    let trees = count_trees(&lines, 1, 3);
    log::info!("Part 1 = {}", trees);

    let combinations = &[(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let trees: u64 = combinations
        .iter()
        .map(|(down_by, right_by)| count_trees(&lines, *down_by, *right_by))
        .product();
    log::info!("Part 2 = {}", trees);
}

/// Count the number of trees encountered when stepping down and to the right
/// by specified amounts.
fn count_trees(lines: &[String], down_by: usize, right_by: usize) -> u64 {
    let mut trees = 0;
    let mut index = 0;

    // Go through each line, skipping lines as needed.
    for line in lines.iter().step_by(down_by) {
        // Calculate the distance to the right, wrapping based on the length of
        // the line. Then get the character at the needed index.
        let right = (index * right_by) % line.len();
        let value = line.chars().nth(right).unwrap();

        index += 1;

        if value == TREE {
            trees += 1;
        }
    }

    trees
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_count_trees() {
        advent_of_code::init();

        let lines: Vec<String> = advent_of_code::utils::decode_line(TEST_INPUT);
        let trees = count_trees(&lines, 1, 3);
        assert_eq!(trees, 7);
    }
}
