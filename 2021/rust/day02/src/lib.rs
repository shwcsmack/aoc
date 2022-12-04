//cargo run --bin part-1
//cargo run --bin part-2
//cargo watch -x check -x test

use std::str::FromStr;

#[derive(Debug)]
struct Command {
    direction: Direction,
    magnitude: u64,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() == 2 {
            let direction = parts[0].parse::<Direction>().unwrap();
            let magnitude = parts[1].parse::<u64>().unwrap();
            Ok(Self{ direction, magnitude})
        } else {
            Err(format!("Bad input: {}", s))
        }
    }
}

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "forward" => Ok(Self::Forward),
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err(format!("command didnt match known commands: {}", s))
        }
    }
}

#[derive(Debug, Default)]
struct Ship {
    depth: u64,
    distance: u64,
}

impl Ship {
    fn process_command(&mut self, command: Command) {
        match command.direction {
            Direction::Forward => self.distance += command.magnitude,
            Direction::Up => self.depth -= command.magnitude,
            Direction::Down => self.depth += command.magnitude,
        }
    }

    fn run_commands(&mut self, commands: Vec<Command>) {
        for command in commands {
            self.process_command(command);
        }
    }
     fn encode(&self) -> u64 {
        self.depth * self.distance
     }
}


pub fn process_part1(input: &str) -> String {
    let commands: Vec<Command> = input
        .lines()
        .into_iter()
        .map(|line| line.trim())
        .filter_map(|line| line.parse::<Command>().ok())
        .collect();
    let mut ship = Ship::default();
    ship.run_commands(commands);
    ship.encode().to_string()
}

pub fn process_part2(input: &str) -> String {
    todo!();
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    #[test]
    fn part_one() {
        let result = process_part1(INPUT);
        assert_eq!(result, "150");
    }

    #[ignore]
    #[test]
    fn part_two() {
        let result = process_part2(INPUT);
        assert_eq!(result, "0")
    }
}
