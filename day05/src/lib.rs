use color_eyre::eyre::{eyre, Result};

pub fn part1(input: &str) -> Result<u32> {
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
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
"
    };

    #[test]
    fn part1_test_input() -> Result<()> {
        assert_eq!(5, part1(TEST_INPUT)?);
        Ok(())
    }

    #[test]
    fn part2_test_input() -> Result<()> {
        assert_eq!("", part2(TEST_INPUT)?);
        Ok(())
    }
}
