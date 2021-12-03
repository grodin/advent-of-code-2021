pub fn part1(input: &str) -> String {
    count_reversals(&get_lines(input)).to_string()
}

pub fn part2(input: &str) -> String {
    let sums: Vec<i32> = get_lines(input)
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();
    count_reversals(&sums).to_string()
}

fn get_lines(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

fn count_reversals(input: &[i32]) -> usize {
    input
        .windows(2)
        .map(|slice| slice[0] - slice[1])
        .filter(|&n: &i32| n < 0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
		199
		200
		208
		210
		200
		207
		240
		269
		260
		263
		"
    };

    #[test]
    fn part1_test_input() {
        assert_eq!("7", part1(TEST_INPUT))
    }

    #[test]
    fn part2_test_input() {
        assert_eq!("5", part2(TEST_INPUT))
    }
}
