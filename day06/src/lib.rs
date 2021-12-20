use color_eyre::eyre::Result;
use lanternfish::LanternFish;

mod lanternfish;

pub fn part1(input: &str) -> Result<u64> {
    solution(input, 80)
}

pub fn part2(input: &str) -> Result<u64> {
    solution(input, 256)
}

fn solution(input: &str, num_days_to_simulate: usize) -> Result<u64> {
    let fish = parse_list_of_fish(input)?;
    let mut fish_count_by_age = [0; 9];
    for f in &fish {
        fish_count_by_age[f.age] += 1;
    }

    for _ in 0..num_days_to_simulate {
        let num_new_fish = fish_count_by_age[0];
        // Fish that are resetting will be added back in after the loop
        fish_count_by_age[0] = 0;

        for state in 1..9 {
            let fish_to_move = fish_count_by_age[state];
            fish_count_by_age[state - 1] += fish_to_move;
            fish_count_by_age[state] -= fish_to_move;
        }
        //Reset fish who are spawning new fish to age 6
        fish_count_by_age[6] += num_new_fish;
        //Spawn new fish
        fish_count_by_age[8] = num_new_fish;
    }
    Ok(fish_count_by_age.iter().sum())
}

fn parse_list_of_fish(input: &str) -> Result<Vec<LanternFish>> {
    input
        .trim()
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
        assert_eq!(26_984_457_539, part2(TEST_INPUT)?);
        Ok(())
    }
}
