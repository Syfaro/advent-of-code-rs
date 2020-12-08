const PROBLEM_NAME: &str = "2015-02";

/// A package's three dimensions.
#[derive(Clone, Debug, PartialEq)]
struct Package {
    length: i32,
    width: i32,
    height: i32,
}

#[cfg(test)]
impl Package {
    fn new(length: i32, width: i32, height: i32) -> Self {
        Self {
            length,
            width,
            height,
        }
    }
}

impl Package {
    /// Calculate the amount of wrapping paper required for the package.
    fn wrapping_paper(&self) -> i32 {
        // Calculate base area of the paper needed.
        let area = (2 * self.length * self.width)
            + (2 * self.width * self.height)
            + (2 * self.height * self.length);

        // Calculate each side to find the side with the least area.
        let smallest = vec![
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ]
        .into_iter()
        .min()
        .unwrap();

        area + smallest
    }

    /// Calculate the amount of ribbon needed for the package.
    fn ribbon(&self) -> i32 {
        // Sort the dimensions to find the two smallest edges.
        let mut faces = vec![self.length, self.width, self.height];
        faces.sort();

        // These will always have 3 values.
        let ribbon = faces[0] * 2 + faces[1] * 2;
        let bow: i32 = faces.iter().product();

        ribbon + bow
    }
}

fn main() {
    advent_of_code::init();

    let input = advent_of_code::load_input(PROBLEM_NAME);
    let packages = decode_all_packages(&input);

    let total_paper = total_wrapping_paper(&packages);
    log::info!("Part 1 = {}", total_paper);

    let total_ribbon = total_ribbon(&packages);
    log::info!("Part 2 = {}", total_ribbon);
}

/// Decode each line into a package.
fn decode_all_packages(input: &str) -> Vec<Package> {
    input.lines().map(decode_package).collect()
}

/// Calculate total amount of wrapping paper needed.
fn total_wrapping_paper(packages: &[Package]) -> i32 {
    packages.iter().map(Package::wrapping_paper).sum()
}

/// Calculate total amount of ribbon needed.
fn total_ribbon(packages: &[Package]) -> i32 {
    packages.iter().map(Package::ribbon).sum()
}

/// Decode a Package from a a length x width x height formatted line.
///
/// # Panics
///
/// Will panic if there are not 3 numeric dimensions separated by an 'x' on each
/// line.
fn decode_package(line: &str) -> Package {
    let dimensions: Vec<&str> = line.split('x').collect();
    assert_eq!(dimensions.len(), 3, "Package must have 3 dimensions");

    let dimensions: Vec<i32> = dimensions
        .into_iter()
        .map(|dimension| {
            dimension
                .parse()
                .expect("All package dimensions must be numbers")
        })
        .collect();

    // We know there are three dimensions in here because of previous assertion.
    Package {
        length: dimensions[0],
        width: dimensions[1],
        height: dimensions[2],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_all_packages() {
        let input = "2x3x4\n1x1x10\n";
        let packages = decode_all_packages(&input);
        assert_eq!(
            packages,
            vec![Package::new(2, 3, 4), Package::new(1, 1, 10)]
        );
    }

    #[test]
    fn test_total_wrapping_paper() {
        let packages = &[Package::new(2, 3, 4), Package::new(1, 1, 10)];
        let total_paper = total_wrapping_paper(packages);
        assert_eq!(total_paper, 101);
    }

    #[test]
    fn test_total_ribbon() {
        let packages = &[Package::new(2, 3, 4), Package::new(1, 1, 10)];
        let total_ribbon = total_ribbon(packages);
        assert_eq!(total_ribbon, 48);
    }

    #[test]
    fn test_decode_package() {
        let package = decode_package("1x2x3");
        assert_eq!(package, Package::new(1, 2, 3));
    }

    #[test]
    fn test_package_wrapping_paper() {
        let paper = Package::new(2, 3, 4).wrapping_paper();
        assert_eq!(paper, 58);

        let paper = Package::new(1, 1, 10).wrapping_paper();
        assert_eq!(paper, 43);
    }

    #[test]
    fn test_package_ribbon() {
        let ribbon = Package::new(2, 3, 4).ribbon();
        assert_eq!(ribbon, 34);

        let ribbon = Package::new(1, 1, 10).ribbon();
        assert_eq!(ribbon, 14);
    }
}
