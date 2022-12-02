use adventofcode_rs::read_lines;
use eyre::{eyre, Result};
use log::{debug, info};
use std::str::FromStr;

fn main() -> Result<()> {
    // Init logging
    env_logger::init();

    info!("Day 02 - Part 2");
    // Read the input file
    let lines = read_lines("input/day02.txt")?;

    // Store the score of the opponent
    let mut opponent_score = 0;
    // Store my score
    let mut my_score = 0;

    // Iterate over the lines
    for line in lines {
        let line = line?;
        // Split the line to retrieve opponent move and my recommended move
        let (opponent_move, my_recommended_strategy) = line.split_once(' ').unwrap();
        // Parse the opponent move
        let opponent_move: Move = opponent_move.parse()?;
        // Parse the my recommended move
        let my_recommended_strategy: Strategy = my_recommended_strategy.parse()?;
        // Compute the move to play
        let my_recommended_move =
            find_move_to_match_strategy(&opponent_move, &my_recommended_strategy);
        debug!(
            "Player 1 move: {:?}, Player 2 move: {:?}",
            opponent_move, my_recommended_move
        );
        // Compute the result of the game
        let round_outcome = opponent_move.fight(&my_recommended_move);
        debug!("Round outcome: {:?}", round_outcome);

        // Score accounting
        // The score for a single round is the score for the shape the player selected
        // (1 for Rock, 2 for Paper, and 3 for Scissors),
        // plus the score for the outcome of the round
        // (0 if you lost, 3 if the round was a draw, and 6 if you won)
        // Update the score of the opponent
        opponent_score += round_outcome.score(PlayerId::Player1) + opponent_move.shape_score();
        // Update my score
        my_score += round_outcome.score(PlayerId::Player2) + my_recommended_move.shape_score();
    }

    // Print the result
    info!("My score: {}", my_score);
    info!("Opponent score: {}", opponent_score);

    Ok(())
}

/// Find the best move to play against the opponent according to the strategy
/// # Arguments
/// * `opponent_move` - The move played by the opponent
/// * `strategy` - The strategy to use
/// # Returns
/// The best move to play
fn find_move_to_match_strategy(opponent_move: &Move, strategy: &Strategy) -> Move {
    match (opponent_move, strategy) {
        // If we need to perform a draw, we play the same move as the opponent
        (_, Strategy::Draw) => opponent_move.clone(),
        (Move::Rock, Strategy::Lose) => Move::Scissors,
        (Move::Rock, Strategy::Win) => Move::Paper,
        (Move::Paper, Strategy::Lose) => Move::Rock,
        (Move::Paper, Strategy::Win) => Move::Scissors,
        (Move::Scissors, Strategy::Lose) => Move::Paper,
        (Move::Scissors, Strategy::Win) => Move::Rock,
    }
}

/// Enum representing the three possible moves in Rock Paper Scissors
#[derive(Debug, PartialEq, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    /// Play a game of Rock, Paper, Scissors
    /// # Arguments
    /// * `player_1_move` - The move of the first player
    /// * `player_2_move` - The move of the second player
    /// # Returns
    /// The round outcome
    fn fight(&self, player_2_move: &Move) -> RoundOutcome {
        match (self, player_2_move) {
            // Rock beats Scissors: Player 1 wins
            (Move::Rock, Move::Scissors) => RoundOutcome::Win(PlayerId::Player1),
            // Paper beats Rock: Player 2 wins
            (Move::Rock, Move::Paper) => RoundOutcome::Win(PlayerId::Player2),
            // Paper beats rock: Player 1 wins
            (Move::Paper, Move::Rock) => RoundOutcome::Win(PlayerId::Player1),
            // Scissors beats Paper: Player 2 wins
            (Move::Paper, Move::Scissors) => RoundOutcome::Win(PlayerId::Player2),
            // Scissors beats Paper: Player 1 wins
            (Move::Scissors, Move::Paper) => RoundOutcome::Win(PlayerId::Player1),
            // Rock beats Scissors: Player 2 wins
            (Move::Scissors, Move::Rock) => RoundOutcome::Win(PlayerId::Player2),
            // If the moves are the same, it's a draw
            _ => RoundOutcome::Draw,
        }
    }

    /// Return the score of the shape
    fn shape_score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

/// Enum representing the different players in Rock Paper Scissors
/// Use `PartialEq` to compare player ids against the winner of a round
#[derive(Debug, PartialEq)]
enum PlayerId {
    Player1,
    Player2,
}

/// Enum representing the three possible outcomes of a Rock Paper Scissors game
#[derive(Debug)]
enum RoundOutcome {
    Win(PlayerId),
    Draw,
}

/// Implementation of `RoundOutcome` for RockPaperScissors game
impl RoundOutcome {
    /// Compute the score of a round for a given player
    /// # Arguments
    /// * `player_id` - The player id
    /// # Returns
    /// The score of the round for the given player
    fn score(&self, player_id: PlayerId) -> u32 {
        match self {
            RoundOutcome::Win(winner) => {
                // WIN
                // If the winner is the given player, the score is 6
                if *winner == player_id {
                    6
                }
                // LOSS
                // If the winner is not the given player, the score is 0
                else {
                    0
                }
            }
            // DRAW
            // If it's a draw, the score is 3, no matter the player
            RoundOutcome::Draw => 3,
        }
    }
}

/// Implement FromStr for `Move`
impl FromStr for Move {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            // If the string is not one of the expected values, return an error
            _ => Err(eyre!("Invalid input")),
        }
    }
    // The error type is eyre::Error
    type Err = eyre::Error;
}

/// Enum representing the three possible strategies
#[derive(Debug, PartialEq)]
enum Strategy {
    Lose,
    Draw,
    Win,
}

/// Implement FromStr for `Strategy`
impl FromStr for Strategy {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "X" => Ok(Strategy::Lose),
            "Y" => Ok(Strategy::Draw),
            "Z" => Ok(Strategy::Win),
            // If the string is not one of the expected values, return an error
            _ => Err(eyre!("Invalid input")),
        }
    }
    // The error type is eyre::Error
    type Err = eyre::Error;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_move_to_match_strategy_should_work() {
        let opponent_move = Move::Rock;
        let strategy = Strategy::Lose;
        let expected_move = Move::Scissors;
        let actual_move = find_move_to_match_strategy(&opponent_move, &strategy);
        assert_eq!(actual_move, expected_move);
    }
}
