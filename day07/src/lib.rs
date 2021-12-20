use color_eyre::eyre::{eyre, Context, Result};

pub fn part1(input: &str) -> Result<u32> {
    fn calculate_fuel_cost_pt1(
        count: &u16,
        original_position: &usize,
        destination_position: &usize,
    ) -> Result<u32> {
        let count = *count as i128;
        let orig_pos = *original_position as i128;
        let dest_pos = *destination_position as i128;
        Ok((count * (orig_pos - dest_pos).abs()).try_into()?)
    }
    solution(input, calculate_fuel_cost_pt1)
}

pub fn part2(input: &str) -> Result<u32> {
    fn calculate_fuel_cost_pt2(
        count: &u16,
        original_position: &usize,
        destination_position: &usize,
    ) -> Result<u32> {
        let count = *count as i128;
        let orig_pos = *original_position as i128;
        let dest_pos = *destination_position as i128;
        let cost = (orig_pos - dest_pos).abs();
        let cost = (cost * (cost + 1)) / 2;
        Ok((count * cost).try_into()?)
    }
    solution(input, calculate_fuel_cost_pt2)
}

fn solution<F: Fn(&u16, &usize, &usize) -> Result<u32>>(
    input: &str,
    calculate_fuel_cost: F,
) -> Result<u32> {
    let count_by_position = parse_input(input)?;
    let mut fuel_champion = u32::MAX;
    for i in 0..count_by_position.len() {
        let fuel_candidate: u32 = count_by_position
            .iter()
            .enumerate()
            .map(|(pos, count)| calculate_fuel_cost(count, &pos, &i))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .sum();
        if fuel_candidate < fuel_champion {
            fuel_champion = fuel_candidate
        }
    }
    Ok(fuel_champion)
}

type CountByPosition = Box<[u16]>;

fn parse_input(input: &str) -> Result<CountByPosition> {
    let initial_positions = input
        .split(',')
        .map(|s| s.trim().parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .wrap_err_with(|| "Error while parsing")?;
    let max_position = initial_positions
        .iter()
        .max()
        .ok_or_else(|| eyre!("Error finding max initial position"))?
        .clone() as usize;
    let mut count_by_position = vec![0_u16; max_position + 1];
    for pos in initial_positions {
        count_by_position[pos] += 1;
    }

    Ok(count_by_position.into_boxed_slice())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"16,1,2,0,4,2,7,1,2,14"};

    #[test]
    fn part1_test_input() -> Result<()> {
        assert_eq!(37, part1(TEST_INPUT)?);
        Ok(())
    }

    #[test]
    fn part2_test_input() -> Result<()> {
        assert_eq!(168, part2(TEST_INPUT)?);
        Ok(())
    }
}
