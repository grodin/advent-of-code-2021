use day01::{part1,part2};

use color_eyre::eyre::Result;

fn main() -> Result<()>{
    let input = include_str!("input.txt");

    part1(input)?;

    Ok(())
}
