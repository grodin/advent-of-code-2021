use color_eyre::eyre::{eyre, Result};

pub fn part1(input: &str) -> Result<String> {
    Err(eyre!("Not implemented"))
}

pub fn part2(input: &str) -> Result<String> {
    Err(eyre!("Not implemented"))
}

fn parse_input(input: &str) -> Result<Vec<_>> {
    input
        .lines()
        .map(|s| s.trim())
        .collect::<Result<Vec<_>, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"16,1,2,0,4,2,7,1,2,14"};

    #[test]
    fn part1_test_input() -> Result<()> {
        assert_eq!("", part1(TEST_INPUT)?);
        Ok(())
    }

    #[test]
    fn part2_test_input() -> Result<()> {
        assert_eq!("", part2(TEST_INPUT)?);
        Ok(())
    }
}
