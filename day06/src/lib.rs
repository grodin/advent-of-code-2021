use color_eyre::eyre::{eyre, Result};

pub fn part1(input: &str) -> Result<usize> {
    Err(eyre!("Not implemented"))
}

pub fn part2(input: &str) -> Result<String> {
    Err(eyre!("Not implemented"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        3,4,3,1,2
		"
    };

    #[test]
    fn part1_test_input() -> Result<()> {
        assert_eq!(5934, part1(TEST_INPUT)?);
        Ok(())
    }

    #[test]
    fn part2_test_input() -> Result<()> {
        assert_eq!("", part2(TEST_INPUT)?);
        Ok(())
    }
}
