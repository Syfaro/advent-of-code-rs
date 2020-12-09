use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

static PROBLEM_NAME: &str = "2020-07";

static DESIRED_BAG: &str = "shiny gold";

lazy_static! {
    static ref BAG_COLOR: Regex = Regex::new(r#"^(?P<color>\w+ \w+) bags?"#).unwrap();
    static ref BAG_CHILDREN: Regex =
        Regex::new(r#"(?P<count>\d+) (?P<color>\w+ \w+) bags?"#).unwrap();
}

fn main() {
    advent_of_code::init();

    let input = advent_of_code::load_input(PROBLEM_NAME);
    let bags = parse_bags(&input);

    let can_contain = bags
        .keys()
        .filter(|key| *key != DESIRED_BAG)
        .filter(|color| bag_can_contain(&bags, color))
        .count();
    log::info!("Part 1 = {}", can_contain);

    let child_bags = child_bags(&bags, DESIRED_BAG);
    log::info!("Part 2 = {}", child_bags);
}

/// Parse all bags into a hash map with information about what and how many
/// bags they can contain.
fn parse_bags(input: &str) -> HashMap<String, Vec<(String, usize)>> {
    let mut bags = HashMap::new();

    for line in input.lines() {
        let color = BAG_COLOR.captures(&line).unwrap()["color"].to_string();
        let can_contain: Vec<_> = BAG_CHILDREN
            .captures_iter(&line)
            .map(|capture| {
                let count: usize = capture["count"].parse().unwrap();
                let color = capture["color"].to_string();

                (color, count)
            })
            .collect();

        bags.insert(color, can_contain);
    }

    bags
}

/// Determine if a bag is capable of holding another bag, either directly or
/// through its children.
fn bag_can_contain(bags: &HashMap<String, Vec<(String, usize)>>, color: &str) -> bool {
    // If we're at the desired bag, we can hold it!
    if color == DESIRED_BAG {
        return true;
    }

    // Get our children to see if any of them can hold it.
    let children = match bags.get(color) {
        Some(children) => children,
        None => return false,
    };

    // Check children recursively to see if they are it or if they can hold it.
    for (child_color, _count) in children {
        if child_color == DESIRED_BAG {
            return true;
        }

        if bag_can_contain(&bags, child_color) {
            return true;
        }
    }

    false
}

/// Calculate the maximum number of child bags that can be held by a parent bag.
fn child_bags(bags: &HashMap<String, Vec<(String, usize)>>, color: &str) -> usize {
    let children = match bags.get(color) {
        Some(children) => children,
        None => return 0,
    };

    // Each bag contains a specific number of direct children and all of the
    // children within those bags.
    children.iter().fold(0, |acc, (color, count)| {
        acc + count + count * child_bags(&bags, &color)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str =
        "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    static OTHER_TEST_INPUT: &'static str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn test_parse_bags() {
        advent_of_code::init();

        let bags = parse_bags(TEST_INPUT);
        assert_eq!(bags.len(), 9);
        assert_eq!(bags["light red"].len(), 2);
        assert_eq!(bags["light red"][1].0, "muted yellow");
        assert_eq!(bags["light red"][1].1, 2);
    }

    #[test]
    fn test_bag_can_contain() {
        advent_of_code::init();

        let bags = parse_bags(TEST_INPUT);
        assert_eq!(bag_can_contain(&bags, "shiny gold"), true);
        assert_eq!(bag_can_contain(&bags, "bright white"), true);
        assert_eq!(bag_can_contain(&bags, "faded blue"), false);
    }

    #[test]
    fn test_child_bags() {
        advent_of_code::init();

        let bags = parse_bags(OTHER_TEST_INPUT);
        assert_eq!(child_bags(&bags, DESIRED_BAG), 126);
    }
}
