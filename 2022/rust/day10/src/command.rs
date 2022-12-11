use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Noop,
    Addx(i64),
}

impl Command {
    pub fn list_from_str(str: &str) -> Vec<Command> {
        str.lines().filter_map(|s| s.parse().ok()).collect()
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split(' ').collect();
        if s.len() > 1 && s[0] == "addx" {
            let result = s[1].parse::<i64>();
            match result {
                Ok(res) => Ok(Self::Addx(res)),
                Err(err) => Err(format!("Couldnt parse into int: {}", err)),
            }
        } else if s.len() == 1 && s[0] == "noop" {
            Ok(Self::Noop)
        } else {
            Err(format!("Couldnt parse into Command: {:?}", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command::*;

    #[test]
    fn command_from_str() {
        let noop_str = "noop";
        let add_str = "addx 5";
        let add_str_2 = "addx -5";

        let noop_comm: Command = noop_str.parse().unwrap();
        let add_comm: Command = add_str.parse().unwrap();
        let add_comm_2: Command = add_str_2.parse().unwrap();

        assert_eq!(noop_comm, Command::Noop);
        assert_eq!(add_comm, Command::Addx(5));
        assert_eq!(add_comm_2, Command::Addx(-5));
    }

    #[test]
    fn list_from_str() {
        let str = "noop
addx 3
addx -5";

        let commands: Vec<Command> = Command::list_from_str(str);

        assert_eq!(commands.len(), 3);
        assert_eq!(commands[0], Command::Noop);
        assert_eq!(commands[1], Command::Addx(3));
        assert_eq!(commands[2], Command::Addx(-5));
    }
}
