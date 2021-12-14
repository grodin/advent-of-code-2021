use color_eyre::eyre::{eyre, Result};

type Cell = (u32, bool);
type Board = [[Cell; 5]; 5];

pub fn part1(input: &str) -> Result<u32> {
    let mut lines = input.lines();
    let moves = parse_moves(lines.next().ok_or_else(|| eyre!("No lines of input!"))?)?;
    let mut boards = parse_boards(&lines.collect::<Vec<_>>().chunks(6).collect::<Vec<_>>())?;
    let mut winner: Option<(u32, Board)> = None;

    'moves: for m in moves.iter() {
        for board in boards.iter_mut() {
            for row in board.iter_mut() {
                for (entry, selected) in row.into_iter() {
                    if !*selected && entry == m {
                        *selected = true;
                    }
                }
            }
            if is_board_a_winner(&board) {
                winner = Some((*m, *board));
                break 'moves;
            }
        }
    }
    winner
        .map(|(m, board)| {
            let sum_of_unselected: u32 = board
                .iter()
                .flat_map(|row| row.iter().filter(|(_, selected)| !selected))
                .map(|(n, _)| n)
                .sum();
            m * sum_of_unselected
        })
        .ok_or_else(|| eyre!("No winning board found"))
}

fn is_board_a_winner(board: &Board) -> bool {
    for &row in board {
        if row.iter().all(|&(_, selected)| selected) {
            return true;
        }
    }
    for i in 0..5 {
        if board
            .iter()
            .map(|&row| row[i])
            .all(|(_, selected)| selected)
        {
            return true;
        }
    }
    false
}

fn parse_moves(line: &str) -> Result<Vec<u32>> {
    let moves = line
        .split(',')
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()?;
    Ok(moves)
}

fn parse_boards(chunks: &[&[&str]]) -> Result<Vec<Board>> {
    let mut boards: Vec<Board> = vec![];
    for &chunk in chunks {
        let mut board: Vec<[Cell; 5]> = vec![];
        for &line in chunk.iter().skip(1) {
            let row = line
                .split_whitespace()
                .map(|entry| -> Result<_, _> { Ok((entry.parse::<u32>()?, false)) })
                .collect::<Result<Vec<Cell>>>()?;
            let row: [Cell; 5] = row.try_into().unwrap();

            board.push(row);
        }
        boards.push(board.try_into().unwrap());
    }
    // chunks
    //     .map(|&lines| lines.iter().skip(1).map(|line|
    //         line.split_whitespace().map(|entry|(entry.parse::<u32>(), false))).collect::<Vec<_>>()
    //     .try_into();).collect::<Vec<_>>()
    //     .try_into();
    //     ).collect();
    Ok(boards)
}

pub fn part2(input: &str) -> Result<String> {
    Err(eyre!("Not implemented"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19

             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7
		"
    };

    #[test]
    fn part1_test_input() -> Result<()> {
        assert_eq!(4512, part1(TEST_INPUT)?);
        Ok(())
    }

    #[test]
    fn part2_test_input() -> Result<()> {
        assert_eq!("", part2(TEST_INPUT)?);
        Ok(())
    }
}
