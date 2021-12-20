use color_eyre::eyre::{eyre, Result};
use lanternfish::LanternFish;

mod lanternfish;

pub fn part1(input: &str) -> Result<usize> {
    let mut fish = parse_list_of_fish(input)?;
    let mut new_fish: Vec<LanternFish> = vec![];
    for _ in 0..80 {
        for f in &mut fish {
            if let Some(new_f) = f.step_day() {
                new_fish.push(new_f);
            }
        }
        fish.append(&mut new_fish);
    }
    Ok(fish.len())
}

pub fn part2(input: &str) -> Result<String> {
    Err(eyre!("Not implemented"))
}

fn parse_list_of_fish(input: &str) -> Result<Vec<LanternFish>> {
    input
        .split(',')
        .map(|s| s.parse::<LanternFish>())
        .collect::<Result<Vec<_>, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"3,4,3,1,2"};

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
