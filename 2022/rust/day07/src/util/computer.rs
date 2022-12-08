use std::{collections::BTreeMap, str::FromStr};

use super::file_system::FileSystem;

#[derive(Debug)]
struct Command {
    script: Script,
    arg: String,
}

impl Command {
    fn is_a_command(s: &str) -> bool {
        s.len() > 1 && &s[0..1] == "$"
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let script = parts[1].parse()?;
        let mut arg = "";
        if parts.len() > 2 {
            arg = parts[2]
        }

        Ok(Self {
            script,
            arg: arg.to_string(),
        })
    }
}

#[derive(Debug)]
enum Script {
    LS,
    CD,
}

impl FromStr for Script {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ls" => Ok(Self::LS),
            "cd" => Ok(Self::CD),
            _ => Err(format!("script not found: {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Path(Vec<String>);

impl Path {
    pub fn new(s: &str) -> Self {
        Path(vec![s.to_string()])
    }
    fn add_path(&mut self, dir: &str) {
        self.0.push(dir.to_string());
    }

    fn go_back(&mut self) {
        self.0.pop();
    }

    fn go_to_root(&mut self) {
        self.0.clear();
        self.0.push("/".to_string());
    }

    fn pwd(&mut self) -> String {
        let pwd = self
            .0
            .clone()
            .iter_mut()
            .reduce(|acc, s| {
                s.push('/');
                acc.push_str(s);
                acc
            })
            .unwrap()
            .to_string();
        pwd
    }
}

impl FromStr for Path {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        let mut path = Path::default();
        path.add_path("/");
        for dir in parts {
            if dir.is_empty() {
                continue;
            }
            path.add_path(dir);
        }
        Ok(path)
    }
}

#[derive(Debug, Default)]
pub struct Computer {
    file_system: BTreeMap<String, String>,
    pwd: Path,
}
impl Computer {
    pub fn process_line(&mut self, line: &str) {
        if Command::is_a_command(line) {
            let command: Command = line.parse().unwrap();
            self.process_command(command);
        } else {
            self.process_output(line);
        }
    }

    fn process_command(&mut self, command: Command) {
        match command.script {
            Script::LS => (),
            Script::CD => {
                if command.arg == ".." {
                    self.pwd.go_back();
                } else if command.arg == "/" {
                    self.pwd.go_to_root();
                } else {
                    self.pwd.add_path(&command.arg);
                }
            }
        }
    }

    fn process_output(&mut self, line: &str) {
        self.file_system.insert(line.to_string(), self.pwd.pwd());
    }

    pub fn size_score(&self) -> u64 {
        let fs = FileSystem::build(self.file_system.clone());
        dbg!(fs);
        0
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part_one() {
        // let result = process_part1(INPUT);
        // assert_eq!(result, "95437");
    }
}
