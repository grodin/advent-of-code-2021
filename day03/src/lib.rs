use bit_field::BitField;
use color_eyre::eyre::{eyre, Result};

pub fn part1(input: &str) -> Result<u32> {
    let (readings, width) = parse(input)?;
    let count_set_bits = compute_bit_counts(width, &readings);
    let total = readings.len();
    let mut gamma = 0_u16;
    let mut epsilon = 0_u16;
    for (i, n) in count_set_bits.into_iter().rev().enumerate() {
        gamma.set_bit(i, n > total - n);
        epsilon.set_bit(i, n < total - n);
    }
    Ok(u32::from(gamma) * u32::from(epsilon))
}

pub fn part2(input: &str) -> Result<u32> {
    let (readings, width) = parse(input)?;

    let o2_rating = find_rating(width, readings.clone(), |total, n| n >= total - n)?;
    let co2_rating = find_rating(width, readings.clone(), |total, n| n < total - n)?;
    Ok(u32::from(o2_rating) * u32::from(co2_rating))
}

fn find_rating<F>(width: usize, mut candidates: Vec<u16>, test: F) -> Result<u16>
where
    F: Fn(usize, usize) -> bool,
{
    for i in (0..width).rev() {
        let n = count_ith_bits(i, &candidates);
        let total = candidates.len();
        candidates.retain(|c| c.get_bit(i) == test(total, n));
        if candidates.len() <= 1 {
            break;
        }
    }
    candidates
        .into_iter()
        .next()
        .ok_or_else(|| eyre!("No o2 rating candidates"))
}

fn parse(input: &str) -> Result<(Vec<u16>, usize)> {
    let readings = input
        .lines()
        .map(|l| u16::from_str_radix(l, 2))
        .collect::<Result<Vec<_>, _>>()?;
    let width = input.lines().next().ok_or_else(|| eyre!("No input"))?.len();
    Ok((readings, width))
}

fn compute_bit_counts(width: usize, readings: &[u16]) -> Vec<usize> {
    let mut count_set_bits = vec![0_usize; width];
    for i in 0..width {
        count_set_bits[width - i - 1] = count_ith_bits(i, readings)
    }
    count_set_bits
}

fn count_ith_bits(i: usize, data: &[u16]) -> usize {
    data.iter()
        .fold(0, |accum, &n| if n.get_bit(i) { accum + 1 } else { accum })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
		00100
		11110
		10110
		10111
		10101
		01111
		00111
		11100
		10000
		11001
		00010
		01010
		"
    };

    #[test]
    fn part1_test_input() -> Result<()> {
        assert_eq!(198, part1(TEST_INPUT)?);
        Ok(())
    }

    #[test]
    fn part2_test_input() -> Result<()> {
        assert_eq!(230, part2(TEST_INPUT)?);
        Ok(())
    }
}
