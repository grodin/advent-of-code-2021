use color_eyre::eyre::Result;
use day04::{part1, part2};

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input)?);

    println!("Part 2: {}", part2(input)?);

    Ok(())
}
