use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

static PROBLEM_NAME: &str = "2020-06";

fn main() {
    advent_of_code::init();

    let input = advent_of_code::load_input(PROBLEM_NAME);
    let groups = parse_groups(&input);

    let unique_questions: usize = groups.iter().map(|group| unique_questions(&group)).sum();
    log::info!("Part 1 = {}", unique_questions);

    let all_answered_yes: usize = groups.iter().map(|group| all_answered(&group)).sum();
    log::info!("Part 2 = {}", all_answered_yes);
}

/// Parse a collection of groups into each group, each person, and each question
/// the person answered yes to.
fn parse_groups(input: &str) -> Vec<Vec<HashSet<char>>> {
    let mut groups = Vec::new();
    let mut group = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            groups.push(group.clone());
            group = Vec::new();
            continue;
        }

        group.push(HashSet::from_iter(line.chars()));
    }

    groups.push(group);

    groups
}

/// Calculate the number of unique questions asked within a group.
fn unique_questions(answers: &[HashSet<char>]) -> usize {
    // All we have to do is construct a HashSet, try to insert every question,
    // then count how many elements we ended up with.
    let mut questions = HashSet::new();

    for group in answers {
        for question in group {
            questions.insert(question);
        }
    }

    questions.len()
}

/// Calculate the number of questions where everyone answered yes.
fn all_answered(answers: &[HashSet<char>]) -> usize {
    // Count the number of times each question has been answered within a group.
    // Once everything is counted, find the number of items who have as many
    // answers as there were people in the group.
    let mut counts: HashMap<char, usize> = HashMap::new();

    for group in answers {
        for question in group {
            let entry = counts.entry(*question).or_default();
            *entry += 1;
        }
    }

    counts
        .iter()
        .filter(|(_question, count)| **count == answers.len())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "abc\n\na\nb\nc\n\nab\nac";

    #[test]
    fn test_parse_groups() {
        advent_of_code::init();

        let groups = parse_groups(TEST_INPUT);
        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0].len(), 1);
        assert_eq!(groups[1].len(), 3);
        assert_eq!(groups[0][0], vec!['a', 'b', 'c'].into_iter().collect());
    }

    #[test]
    fn test_unique_questions() {
        advent_of_code::init();

        let groups = parse_groups(TEST_INPUT);
        let group = &groups[2];
        let unique_questions = unique_questions(&group);
        assert_eq!(unique_questions, 3);
    }

    #[test]
    fn test_all_answered() {
        advent_of_code::init();

        let groups = parse_groups(TEST_INPUT);

        let all_yes = all_answered(&groups[0]);
        assert_eq!(all_yes, 3);

        let all_yes = all_answered(&groups[1]);
        assert_eq!(all_yes, 0);

        let all_yes = all_answered(&groups[2]);
        assert_eq!(all_yes, 1);
    }
}
