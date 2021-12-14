#![allow(dead_code)]
// We could probably do a time/memory tradeoff here by making a different data structure,
// but not sure if it would be faster in real-life.
#[derive(Clone, Debug)]
struct Board {
    numbers: [[u64; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl<S: AsRef<str>> From<&[S]> for Board {
    fn from(chunk: &[S]) -> Self {
        assert_eq!(chunk.len(), 5);

        let mut numbers = [[0u64; 5]; 5];
        let marked = [[false; 5]; 5];

        for row in 0..5 {
            let splitted: Vec<(usize, u64)> = chunk[row]
                .as_ref()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .enumerate()
                .collect();

            assert_eq!(splitted.len(), 5);
            splitted.iter().for_each(|(col, x)| numbers[row][*col] = *x);
        }

        Board { numbers, marked }
    }
}

impl Board {
    fn check_victory(&self) -> bool {
        for i in 0..5 {
            let mut victory = true;
            for j in 0..5 {
                if !self.marked[i][j] {
                    victory = false;
                    break;
                }
            }
            if victory {
                return true;
            }
        }

        for i in 0..5 {
            let mut victory = true;
            for j in 0..5 {
                if !self.marked[j][i] {
                    victory = false;
                    break;
                }
            }
            if victory {
                return true;
            }
        }

        false
    }
}

fn parse_input<S: AsRef<str>>(chunks: &[Vec<S>]) -> (Vec<u64>, Vec<Board>) {
    let draws = chunks[0][0]
        .as_ref()
        .split(',')
        .map(|draw| draw.parse().unwrap())
        .collect();

    let boards = chunks
        .iter()
        .skip(1)
        .map(|chunk| Board::from(chunk.as_ref()))
        .collect();

    (draws, boards)
}

fn execute_draw(draw: u64, boards: &mut [Board]) {
    for mut board in boards {
        for row in 0..5 {
            for col in 0..5 {
                if board.numbers[row][col] == draw {
                    board.marked[row][col] = true;
                }
            }
        }
    }
}

fn play(draws: &[u64], orig_boards: &[Board]) -> Option<(u64, Board)> {
    let mut boards = orig_boards.to_vec();

    for draw in draws {
        execute_draw(*draw, &mut boards);
        if let Some(winner) = boards.iter().find(|board| board.check_victory()) {
            return Some((*draw, winner.clone()));
        }
    }

    None
}

fn play_until_last(draws: &[u64], orig_boards: &[Board]) -> Option<(u64, Board)> {
    let mut boards = orig_boards.to_vec();
    let mut winning_tuple = None;

    for draw in draws {
        execute_draw(*draw, &mut boards);
        let winners: Vec<usize> = boards
            .iter()
            .enumerate()
            .filter(|(_, board)| board.check_victory())
            .map(|(pos, _)| pos)
            .rev()
            .collect();

        for winner_pos in winners {
            winning_tuple = Some((*draw, boards[winner_pos].clone()));
            boards.remove(winner_pos);
        }
    }

    winning_tuple
}

fn calculate_mul(winning_draw: u64, winning_board: &Board) -> u64 {
    let mut sum = 0;
    for row in 0..5 {
        for col in 0..5 {
            if !winning_board.marked[row][col] {
                sum += winning_board.numbers[row][col]
            }
        }
    }
    sum * winning_draw
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{file_io, parse};

    const TEST_DATA: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
 2  0 12  3  7";

    #[test]
    fn test_check_victory() {
        let nums = [[0u64; 5]; 5];
        let mut marked = [[false; 5]; 5];
        marked[1][1] = true;
        let board_loss = Board {
            numbers: nums,
            marked,
        };
        assert!(!board_loss.check_victory());

        let mut board_row_victory = board_loss.clone();
        board_row_victory.marked[2] = [true; 5];
        assert!(board_row_victory.check_victory());

        let mut board_col_victory = board_loss;
        for row in 0..5 {
            board_col_victory.marked[row][2] = true;
        }
        assert!(board_col_victory.check_victory());
    }

    #[test]
    fn test_play() {
        let chunks = parse::split_per_double_newline(&TEST_DATA);
        let (draws, boards) = parse_input(&chunks);
        let result = play(&draws, &boards);
        assert!(result.is_some());
        if let Some((winning_draw, winning_board)) = result {
            assert_eq!(calculate_mul(winning_draw, &winning_board), 4512);
        };
    }

    #[test]
    fn test_play_until_last() {
        let chunks = parse::split_per_double_newline(&TEST_DATA);
        let (draws, boards) = parse_input(&chunks);
        let result = play_until_last(&draws, &boards);
        assert!(result.is_some());
        if let Some((winning_draw, winning_board)) = result {
            assert_eq!(calculate_mul(winning_draw, &winning_board), 1924);
        };
    }

    #[test]
    fn test_d04() {
        let chunks = file_io::read_lines_as_string_groups("inputs/d04").unwrap();
        let (draws, boards) = parse_input(&chunks);
        if let Some((winning_number, winning_board)) = play(&draws, &boards) {
            let result_one = calculate_mul(winning_number, &winning_board);
            println!("Day 04 result #1: {}", result_one);
        } else {
            panic!("Nobody won in part #1!");
        }

        if let Some((winning_number, winning_board)) = play_until_last(&draws, &boards) {
            let result_two = calculate_mul(winning_number, &winning_board);
            println!("Day 04 result #2: {}", result_two);
        } else {
            panic!("Nobody won in part #2!");
        }
    }
}
