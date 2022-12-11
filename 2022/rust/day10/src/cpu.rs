use crate::command::*;

#[derive(Debug)]
pub struct CPU {
    register_x: i64,
    history: Vec<i64>,
}

impl CPU {
    pub fn run_command(&mut self, command: Command) {
        match command {
            Command::Noop => self.history.push(self.register_x),
            Command::Addx(x) => {
                self.history.push(self.register_x);
                self.register_x += x;
                self.history.push(self.register_x);
            }
        }
    }

    pub fn run_command_list(&mut self, commands: Vec<Command>) {
        for command in commands {
            self.run_command(command);
        }
    }

    pub fn signal_strength_at_cycle(&self, cycle: usize) -> i64 {
        dbg!(cycle);
        dbg!(&self.history[cycle - 1]);
        cycle as i64 * self.history[cycle - 1]
    }

    pub fn get_score(&self) -> i64 {
        let num_of_measurments = (self.history.len() - 20) / 40;
        let mut result: Vec<i64> = Vec::new();
        for idx in 0..=num_of_measurments {
            let cycle = 20 + (idx * 40);
            dbg!(cycle);
            let signal_strength = self.signal_strength_at_cycle(cycle);
            dbg!(signal_strength);
            result.push(signal_strength);
        }
        result.iter().sum()
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            register_x: 1,
            history: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::*;

    const SMALL_TEST: &str = "noop
addx 3
addx -5";

    #[test]
    fn cpu_can_be_default() {
        let cpu = CPU::default();

        assert_eq!(cpu.register_x, 1);
        assert_eq!(cpu.history, Vec::default());
    }

    #[test]
    fn cpu_runs_noop_commands() {
        let mut cpu = CPU::default();

        cpu.run_command(Command::Noop);
        assert_eq!(cpu.register_x, 1);
        assert_eq!(cpu.history[0], 1);

        cpu.run_command(Command::Noop);
        assert_eq!(cpu.register_x, 1);
        assert_eq!(cpu.history[0], 1);
        assert_eq!(cpu.history[1], 1);
        assert_eq!(cpu.history.len(), 2);
    }

    #[test]
    fn cpu_runs_add_command() {
        let mut cpu = CPU::default();

        cpu.run_command(Command::Addx(5));
        assert_eq!(cpu.history[0], 1);
        assert_eq!(cpu.history[1], 6);
        assert_eq!(cpu.register_x, 6);

        cpu.run_command(Command::Addx(-5));
        assert_eq!(cpu.history[2], 6);
        assert_eq!(cpu.history[3], 1);
        assert_eq!(cpu.register_x, 1);
    }

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
    fn run_list_of_commands() {
        let mut cpu = CPU::default();
        let commands = Command::list_from_str(SMALL_TEST);
        cpu.run_command_list(commands);

        assert_eq!(cpu.register_x, -1);
        assert_eq!(cpu.history.len(), 5);
        assert_eq!(cpu.history[0], 1);
        assert_eq!(cpu.history[1], 1);
        assert_eq!(cpu.history[2], 4);
        assert_eq!(cpu.history[3], 4);
        assert_eq!(cpu.history[4], -1);
    }

    #[test]
    fn try_get_signal_strength() {
        let mut cpu = CPU::default();
        let commands = Command::list_from_str(SMALL_TEST);
        cpu.run_command_list(commands);

        assert_eq!(cpu.signal_strength_at_cycle(1), 1);
        assert_eq!(cpu.signal_strength_at_cycle(2), 2);
        assert_eq!(cpu.signal_strength_at_cycle(3), 3 * 4);
        assert_eq!(cpu.signal_strength_at_cycle(4), 4 * 4);
        assert_eq!(cpu.signal_strength_at_cycle(5), -5);
    }
}
