use std::str::FromStr;

#[derive(Default, Debug, Clone)]
pub struct BingoBoard {
    pub board: Vec<Vec<(u8, bool)>>,
}
impl BingoBoard {
    pub fn new(board: Vec<Vec<u8>>) -> Self {
        Self {
            board: board
                .into_iter()
                .map(|row| {
                    row
                        .into_iter()
                        .map(|col| (col, false))
                        .collect::<Vec<(u8, bool)>>()
                })
                .collect(),
        }
    }

    pub fn row_complete(&self) -> bool {
        self.board
            .iter()
            .any(|row| {
                row
                    .iter()
                    .all(|(_, hit)| *hit)
            })
    }

    pub fn col_complete(&self) -> bool {
        (0..5)
            .into_iter()
            .any(|i| {
                (0..5)
                    .into_iter()
                    .all(|j| {
                        let (_, hit) = self.board[j][i];
                        hit
                    })
            })
    }

    pub fn has_won(&self) -> bool {
        self.row_complete() || self.col_complete()
    }

    pub fn set_number(&mut self, number: u8) {
        for row in self.board.iter_mut() {
            for (num, hit) in row.iter_mut() {
                if num == &number {
                    *hit = true;
                }
            }
        }
    }

    pub fn calculate_score(&self, last_number: u8) -> u32 {
        self.board
            .iter()
            .map(|row| {
                row
                    .iter()
                    .map(|(num, hit)| {
                        if !hit {
                            *num as u32
                        } else {
                            0 as u32
                        }
                    })
                    .sum::<u32>()
            })
            .sum::<u32>() * last_number as u32
    }
}

pub fn day4_parse(input: &str) -> (Vec<BingoBoard>, Vec<u8>) {
    let (numbers_str, boards_str) = input.split_once("\r\n\r\n").unwrap();
    let numbers_to_draw =
        numbers_str
            .split(',')
            .flat_map(u8::from_str)
            .collect();

    let boards =
        boards_str
            .split("\r\n\r\n")
            .into_iter()
            .map(|board| {
                let bingo_board =
                    board
                    .lines()
                    .map(|line| {
                        line
                            .split(' ')
                            .flat_map(u8::from_str)
                            .collect()
                    })
                    .collect();
                BingoBoard::new(bingo_board)
            })
            .collect();

    (boards, numbers_to_draw)
}

fn draw_number(bingo_boards: &mut Vec<BingoBoard>, drawn_number: u8) {
    for bingo_board in bingo_boards.iter_mut() {
        bingo_board.set_number(drawn_number);
    }
}

fn play(mut bingo_boards: Vec<BingoBoard>, numbers: Vec<u8>, until: u8) -> u32 {
    let mut score = 0_u32;
    let mut has_won = 0_u8;
    let mut board_index = Vec::with_capacity(bingo_boards.len());
    let mut x = 0;

    for (i, number) in numbers.iter().enumerate() {
        draw_number(&mut bingo_boards, *number);

        if i >= 4 {
            for (index, board) in bingo_boards.iter().enumerate() {
                if board.has_won() {
                    has_won += 1;
                    board_index.push(index);
                }

                if has_won >= until {
                    score = board.calculate_score(*number);
                    break;
                }
            }
        }

        if has_won >= until {
            break;
        }


        for index in board_index.iter() {
            bingo_boards.remove(*index - x);
            x += 1;
        }

        x = 0;
        board_index.clear();
    }
    score
}

pub fn day4_1(bingo_game: (Vec<BingoBoard>, Vec<u8>)) -> u32 {
    let (bingo_boards, numbers) = bingo_game;
    let until = 1;

    play(bingo_boards, numbers, until)
}

pub fn day4_2(bingo_game: (Vec<BingoBoard>, Vec<u8>)) -> u32 {
    let (bingo_boards, numbers) = bingo_game;
    let until = bingo_boards.len() as u8;

    play(bingo_boards, numbers, until)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\r\n\r\n22 13 17 11  0\r\n 8  2 23  4 24\r\n21  9 14 16  7\r\n 6 10  3 18  5\r\n 1 12 20 15 19\r\n\r\n 3 15  0  2 22\r\n 9 18 13 17  5\r\n19  8  7 25 23\r\n20 11 10 24  4\r\n14 21 16 12  6\r\n\r\n14 21 17 24  4\r\n10 16 15  9 19\r\n18  8 23 26 20\r\n22 11 13  6  5\r\n 2  0 12  3  7";
    #[test]
    fn d4p1() {
        let day4 = day4_parse(INPUT);
        assert_eq!(day4_1(day4), 4512);
    }

    #[test]
    fn d4p2() {
        let day4 = day4_parse(INPUT);
        assert_eq!(day4_2(day4), 1924);
    }
}