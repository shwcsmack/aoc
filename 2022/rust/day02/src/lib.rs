//cargo run --bin part-1
//cargo run --bin part-2
//cargo watch -x check -x test

enum Outcome {
    Win,
    Draw,
    Lose,
}

#[derive(Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scisors,
}

impl Hand {
    fn parse_elf_hand(str: &str) -> Result<Self, String> {
        match str {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scisors),
            _ => Err(format!("No match for {}", str).to_string()),
        }
    }

    fn parse_my_hand_based_on_elf(str: &str, elf: Hand) -> Result<Self, String> {
        match str {
            "X" => {
                match elf {
                    Hand::Rock => Ok(Hand::Scisors),
                    Hand::Paper => Ok(Hand::Rock),
                    Hand::Scisors => Ok(Hand::Paper),
                }
            },
            "Y" => {
                match elf {
                    Hand::Rock => Ok(Hand::Rock),
                    Hand::Paper => Ok(Hand::Paper),
                    Hand::Scisors => Ok(Hand::Scisors),
                }
            },
            "Z" => {
                match elf {
                    Hand::Rock => Ok(Hand::Paper),
                    Hand::Paper => Ok(Hand::Scisors),
                    Hand::Scisors => Ok(Hand::Rock),
                }
            },
            _ => Err(format!("No match for {}", str).to_string()),
        }
    }
    
    fn outcome_with(&self, other: Hand) -> Outcome {
        match self {
            Hand::Rock => {
                match other {
                    Hand::Rock => Outcome::Draw,
                    Hand::Paper => Outcome::Lose,
                    Hand::Scisors => Outcome::Win,
                }
            },
            Hand::Paper => {
                match other {
                    Hand::Rock => Outcome::Win,
                    Hand::Paper => Outcome::Draw,
                    Hand::Scisors => Outcome::Lose,
                }
            },
            Hand::Scisors => {
                match other {
                    Hand::Rock => Outcome::Lose,
                    Hand::Paper => Outcome::Win,
                    Hand::Scisors => Outcome::Draw,
                }
            },
        }
    }
}

impl std::str::FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "X" || s == "A" {
            Ok(Self::Rock)
        } else if s == "Y" || s == "B" {
            Ok(Self::Paper)
        } else if s == "Z" || s == "C" {
            Ok(Self::Scisors)
        } else {
            Err(format!("No Match from {}", s))
        }
    }
}

#[derive(Clone, Copy)]
struct RPSRound {
    my_hand: Hand,
    elf_hand: Hand,
}

impl RPSRound {
    fn score(&self) -> u32 {
        let my_hand_score: u32 = match self.my_hand {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scisors => 3,
        };
        let outcome = self.my_hand.outcome_with(self.elf_hand);
        match outcome {
            Outcome::Win => 6 + my_hand_score,
            Outcome::Draw => 3 + my_hand_score,
            Outcome::Lose => 0 + my_hand_score,
        }
    }
    
    fn parse_part1(str: &str) -> Result<Self, String> {
        let parts = str.split_whitespace().map(|hand| hand.parse()).collect::<Result<Vec<Hand>, String>>()?;
        Ok(
            Self {
                my_hand: parts[1],
                elf_hand: parts[0],
            }
        )
    }
    
    fn parse_part2(str: &str) -> Result<Self, String> {
        let parts: Vec<&str> = str.split_whitespace().collect();
        let elf_hand = Hand::parse_elf_hand(parts[0])?;
        let my_hand = Hand::parse_my_hand_based_on_elf(parts[1], elf_hand)?;
        Ok(
            Self {
                my_hand,
                elf_hand,
            }
        )
    }


    fn parse_with_strategy(str: &str, strategy: &ParsingStrategy) -> Result<Self, String> {
        match strategy {
            ParsingStrategy::PartOne => RPSRound::parse_part1(str),
            ParsingStrategy::PartTwo => RPSRound::parse_part2(str),
        }
    }
}

enum ParsingStrategy {
    PartOne,
    PartTwo,
}

struct RPSGame {
    rounds: Vec<RPSRound>,
}

impl RPSGame {
    fn from_str_with_strategy(str: &str, strategy: ParsingStrategy) -> Result<Self, String> {
        let rounds: Vec<RPSRound> = str.lines()
        .map(|line| RPSRound::parse_with_strategy(line, &strategy))
        .collect::<Result<Vec<RPSRound>, String>>()?;
        Ok(Self { rounds })
    }

    fn get_score(self) -> u32 {
        self.rounds.into_iter().map(|round| round.score()).sum()
    }
}

pub fn process_part1(input: &str) -> String {
    let game = RPSGame::from_str_with_strategy(input, ParsingStrategy::PartOne).unwrap();
    game.get_score().to_string()
}

pub fn process_part2(input: &str) -> String {
    let game = RPSGame::from_str_with_strategy(input, ParsingStrategy::PartTwo).unwrap();
    game.get_score().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
    B X
    C Z";
    
    #[test]
    fn part_one() {
        let result = process_part1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn part_two() {
        let result = process_part2(INPUT);
        assert_eq!(result, "12");
    }
}
