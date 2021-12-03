use itertools::Itertools;

pub fn part1(input: &str) -> i32 {
    let mut position: i32 = 0;
    let mut depth: i32 = 0;
    let commands = commands(input);
    for (cmd, val) in commands {
        let (pos_delta, depth_delta): (i32, i32) = match cmd {
            "forward" => (val, 0),
            "down" => (0, val),
            "up" => (0, -1 * val),
            _ => panic!("unknown command {}", cmd),
        };
        position += pos_delta;
        depth += depth_delta;
    }
    position * depth
}

pub fn part2(input: &str) -> i32 {
    let mut position: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    let commands = commands(input);
    for (cmd, val) in commands {
        let (pos_delta, depth_delta, aim_delta): (i32, i32, i32) = match cmd {
            "forward" => (val, aim * val, 0),
            "down" => (0, 0, val),
            "up" => (0, 0, -1 * val),
            _ => panic!("unknown command {}", cmd),
        };
        position += pos_delta;
        depth += depth_delta;
        aim += aim_delta;
    }
    position * depth
}

fn commands(input: &str) -> Vec<(&str, i32)> {
    input
        .lines()
        .map(|line| line.split_whitespace().take(2).collect_tuple().unwrap())
        .map(|(cmd, val)| (cmd, val.parse::<i32>().unwrap()))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
		"
    };

    #[test]
    fn part1_test_input() {
        assert_eq!(150, part1(TEST_INPUT))
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(900, part2(TEST_INPUT))
    }
}
