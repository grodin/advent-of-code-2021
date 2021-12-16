mod lines;

use crate::lines::{Line, Point};
use color_eyre::eyre::{eyre, Result};

// type Grid = &[&[usize]];

pub fn part1(input: &str) -> Result<usize> {
    let lines = parse_lines(input)?
        .into_iter()
        .filter(|line| line.is_vertical() || line.is_horizontal())
        .collect::<Vec<_>>();
    count_dangerous_points(&lines)
}

pub fn part2(input: &str) -> Result<usize> {
    let lines = parse_lines(input)?;
    count_dangerous_points(&lines)
}

fn parse_lines(input: &str) -> Result<Vec<Line>> {
    input
        .lines()
        .map(|s| s.parse::<Line>())
        .collect::<Result<Vec<_>, _>>()
}

fn max_x_and_y(lines: &[Line]) -> Result<(usize, usize)> {
    let max_x = lines
        .iter()
        .flat_map(|line| vec![line.initial_point.x, line.terminal_point.x])
        .max()
        .ok_or_else(|| eyre!("Can't find a max x value"))?;
    let max_y = lines
        .iter()
        .flat_map(|line| vec![line.initial_point.y, line.terminal_point.y])
        .max()
        .ok_or_else(|| eyre!("Can't find a max y value"))?;
    Ok((max_x, max_y))
}

fn apply_lines_to_grid(lines: &[Line], grid: &mut [Vec<usize>]) -> Result<()> {
    for line in lines {
        for Point { x, y } in line.points()? {
            grid[y].as_mut_slice()[x] += 1
        }
    }
    Ok(())
}

fn count_dangerous_points(lines: &[Line]) -> Result<usize> {
    let (max_x, max_y) = max_x_and_y(&lines)?;
    let mut grid = vec![vec![0_usize; max_y + 1]; max_x + 1];
    apply_lines_to_grid(&lines, &mut grid)?;

    let dangerous_points = grid
        .into_iter()
        .map(|row| row.into_iter().filter(|&i| i >= 2).count())
        .sum();
    Ok(dangerous_points)
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
        assert_eq!(12, part2(TEST_INPUT)?);
        Ok(())
    }
}
