use std::path::PathBuf;

use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn point_value(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    pub fn losing_response(self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    pub fn winning_response(self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    pub fn vs_my_move(self, my_move: Move) -> Outcome {
        match self {
            opponent if opponent.winning_response() == my_move => Outcome::Win,
            opponent if opponent.losing_response() == my_move => Outcome::Loss,
            _ => Outcome::Tie,
        }
    }

    fn move_needed_for(self, outcome: Outcome) -> Move {
        match outcome {
            Outcome::Loss => self.losing_response(),
            Outcome::Tie => self,
            Outcome::Win => self.winning_response(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Loss,
    Tie,
    Win,
}

impl Outcome {
    pub fn score(self) -> usize {
        match self {
            Outcome::Loss => 0,
            Outcome::Tie => 3,
            Outcome::Win => 6,
        }
    }
}

pub struct Round {
    opponent: Move,
    me: Move,
}

impl Round {
    pub fn my_score(&self) -> usize {
        self.me.point_value() + self.opponent.vs_my_move(self.me).score()
    }
}

pub struct Strategy {
    actions: Vec<Round>,
}

impl Strategy {
    pub fn run_and_score(&self) -> usize {
        self.actions.iter().map(|round| round.my_score()).sum()
    }

    pub fn load(file: impl Into<PathBuf>) -> Result<Strategy, anyhow::Error> {
        let data = std::fs::read_to_string(file.into())?;
        let mut actions = Vec::new();
        for line in data.lines() {
            let mut moves = line.split_whitespace();

            let opponent = moves
                .next()
                .ok_or(anyhow!("No opponent move"))
                .and_then(|input| match input {
                    "A" => Ok(Move::Rock),
                    "B" => Ok(Move::Paper),
                    "C" => Ok(Move::Scissors),
                    _ => Err(anyhow!("Invalid opponent input: {input}")),
                })?;

            let me = moves
                .next()
                .ok_or(anyhow!("No self move"))
                .and_then(|input| match input {
                    "X" => Ok(opponent.move_needed_for(Outcome::Loss)),
                    "Y" => Ok(opponent.move_needed_for(Outcome::Tie)),
                    "Z" => Ok(opponent.move_needed_for(Outcome::Win)),
                    _ => Err(anyhow!("Invalid self input: {input}")),
                })?;

            actions.push(Round { opponent, me });
        }
        Ok(Strategy { actions })
    }

    pub fn load_incorrect(file: impl Into<PathBuf>) -> Result<Strategy, anyhow::Error> {
        let data = std::fs::read_to_string(file.into())?;
        let mut actions = Vec::new();
        for line in data.lines() {
            let mut moves = line.split_whitespace();

            let opponent = moves
                .next()
                .ok_or(anyhow!("No opponent move"))
                .and_then(|input| match input {
                    "A" => Ok(Move::Rock),
                    "B" => Ok(Move::Paper),
                    "C" => Ok(Move::Scissors),
                    _ => Err(anyhow!("Invalid opponent input: {input}")),
                })?;

            let me = moves
                .next()
                .ok_or(anyhow!("No self move"))
                .and_then(|input| match input {
                    "X" => Ok(Move::Rock),
                    "Y" => Ok(Move::Paper),
                    "Z" => Ok(Move::Scissors),
                    _ => Err(anyhow!("Invalid self input: {input}")),
                })?;

            actions.push(Round { opponent, me });
        }
        Ok(Strategy { actions })
    }
}
