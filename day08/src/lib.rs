use color_eyre::eyre::{eyre, Result};

pub fn part1(input: &str) -> Result<u32> {
    Err(eyre!("Not implemented"))
}

pub fn part2(input: &str) -> Result<String> {
    Err(eyre!("Not implemented"))
}

fn parse_input(input: &str) -> Result<Vec<_>> {
    input.lines().map(|s| -> &str { s.trim() })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"};

    #[test]
    fn part1_test_input() -> Result<()> {
        assert_eq!(26, part1(TEST_INPUT)?);
        Ok(())
    }

    #[test]
    fn part2_test_input() -> Result<()> {
        assert_eq!("", part2(TEST_INPUT)?);
        Ok(())
    }
}
