use std::collections::HashSet;

const PROBLEM_NAME: &str = "2015-03";

/// A 2D point.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point(i32, i32);

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

/// A movement direction.
#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Convert from a character into a direction.
    ///
    /// # Panics
    ///
    /// This will panic if an unknown character is provided.
    fn from_char(c: char) -> Direction {
        match c {
            '^' => Direction::North,
            'v' => Direction::South,
            '>' => Direction::East,
            '<' => Direction::West,
            _ => panic!("Unknown direction"),
        }
    }

    /// Movement expressed by this direction, going from the top left to bottom
    /// right.
    fn movement(&self) -> Point {
        match self {
            Direction::North => Point(0, -1),
            Direction::South => Point(0, 1),
            Direction::East => Point(1, 0),
            Direction::West => Point(-1, 0),
        }
    }
}

fn main() {
    advent_of_code::init();

    let input = advent_of_code::load_input(PROBLEM_NAME);
    let directions = decode_directions(&input);

    let unique_locations = find_unique_locations(&directions, 1);
    log::info!("Part 1 = {}", unique_locations.len());

    let inverted_directions = find_unique_locations(&directions, 2);
    log::info!("Part 2 = {}", inverted_directions.len());
}

/// Decode directions from input string.
fn decode_directions(input: &str) -> Vec<Direction> {
    input.chars().map(Direction::from_char).collect()
}

/// Count the number of unique locations visited by having multiple movers,
/// going to the next mover each direction.
fn find_unique_locations(directions: &[Direction], movers: usize) -> HashSet<Point> {
    let mut locations = HashSet::new();
    // We need to include our starting point.
    locations.insert(Point(0, 0));

    // Maintain a point for each mover.
    let mut points = vec![Point(0, 0); movers];

    // Iterate through each direction, looking at which step we're on. Find the
    // correct point to update based on which mover is currently active.
    for (index, direction) in directions.iter().enumerate() {
        let point = &mut points[index % movers];
        *point += direction.movement();
        log::trace!("Evaluating point {}, moved to {}", index % movers, point);
        locations.insert(*point);
    }

    log::debug!("Directions had {} unique points", locations.len());

    locations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_add_assign() {
        advent_of_code::init();

        let mut point = Point(0, 1);
        point += Point(1, 1);
        assert_eq!(point, Point(1, 2));
    }

    #[test]
    fn test_direction_from_char() {
        advent_of_code::init();

        let direction = Direction::from_char('>');
        assert_eq!(direction, Direction::East);
    }

    #[test]
    fn test_decode_directions() {
        advent_of_code::init();

        let directions = decode_directions("><^v");
        assert_eq!(
            directions,
            vec![
                Direction::East,
                Direction::West,
                Direction::North,
                Direction::South
            ]
        );
    }

    #[test]
    fn test_find_unique_locations() {
        advent_of_code::init();

        let cases = &[(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)];
        for (input, count) in cases {
            let directions = decode_directions(input);
            let unique_locations = find_unique_locations(&directions, 1);
            assert_eq!(unique_locations.len(), *count);
        }
    }

    #[test]
    fn test_find_unique_alternating_locations() {
        advent_of_code::init();

        let cases = &[("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)];
        for (input, count) in cases {
            let directions = decode_directions(input);
            let unique_locations = find_unique_locations(&directions, 2);
            assert_eq!(unique_locations.len(), *count);
        }
    }
}
