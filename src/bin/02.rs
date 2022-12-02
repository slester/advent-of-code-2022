/*
A for Rock, B for Paper, and C for Scissors
X for Rock, Y for Paper, and Z for Scissors
(1 for Rock, 2 for Paper, and 3 for Scissors)
 */

#[derive(PartialEq)]
enum Move {
    Rock = 1, // A, X
    Paper = 2, // B, Y
    Scissors = 3, // C, Z
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |mut score: u32, line: &str| {
        let (opponent, me) = line.split_once(" ").unwrap();
        let opponent_move = match opponent {
            "A" => Move::Rock,
            "B" => Move::Paper,
            _ => Move::Scissors,
        };
        let my_move = match me {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            _ => Move::Scissors,
        };

        if opponent_move == Move::Rock && my_move == Move::Paper ||
            opponent_move == Move::Paper && my_move == Move::Scissors ||
            opponent_move == Move::Scissors && my_move == Move::Rock {
            score += 6;
        } else if opponent_move == my_move {
            score += 3;
        }
        score + my_move as u32
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |mut score: u32, line: &str| {
        let (opponent, me) = line.split_once(" ").unwrap();
        let opponent_move = match opponent {
            "A" => Move::Rock,
            "B" => Move::Paper,
            _ => Move::Scissors,
        };

        let my_move: Move;
        score += match me {
            // lose
            "X" => {
                if opponent_move == Move::Rock {
                    my_move = Move::Scissors;
                } else if opponent_move == Move::Paper {
                    my_move = Move::Rock;
                } else {
                    my_move = Move::Paper;
                }
                0
            },
            // draw
            "Y" => {
                my_move = opponent_move;
                3
            },
            // win
            _ => {
                if opponent_move == Move::Rock {
                    my_move = Move::Paper;
                } else if opponent_move == Move::Paper {
                    my_move = Move::Scissors;
                } else {
                    my_move = Move::Rock;
                }
               6
            },
        };

        score + my_move as u32
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
