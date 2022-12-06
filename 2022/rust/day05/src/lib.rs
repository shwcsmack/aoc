//cargo run --bin part-1
//cargo run --bin part-2
//cargo watch -x check -x test

use std::{str::FromStr, collections::VecDeque};

mod parser;

#[derive(Debug, Clone, Copy)]
struct SupplyBox(Option<char>);

impl FromStr for SupplyBox {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len()==3 {
            let chars: Vec<char> = s.chars().collect();
            match chars[0] {
                '[' => Ok(Self(Some(s.chars().nth(1).unwrap()))),
                ' ' => Ok(Self(None)),
                _ => Err(format!("Bad input(couldnt parse into box: {}", s))
            }
        } else {
            Err(format!("Poorly formed input(should be 3 chars): {}", s))
        }
    }
}

#[derive(Debug)]
struct Warehouse{
    width: usize,
    stacks: Vec<BoxStack>,
}

impl Warehouse {
    fn new(boxes: Vec<SupplyBox>, width: usize) -> Self {
        let height = boxes.len() / width;
        let mut stacks: Vec<BoxStack> = Vec::new();
        for col in 0..width {
            let mut stack = BoxStack::default();
            for row in 0..height {
                let this_box = &boxes[row*width + col];
                if this_box.0.is_some() {
                    stack.0.push_back(this_box.0.unwrap().clone())
                }
            }
            stacks.push(stack);
        }
        Self { width, stacks }
    }

    fn run_command(&mut self, command: Command) {
        for _ in 0..command.num_of_boxes {
            let temp_box = self.stacks[(command.from_idx-1) as usize].0.pop_front().unwrap();
            self.stacks[(command.to_idx - 1) as usize].0.push_front(temp_box);
        }
    }

    fn run_command_2(&mut self, command: Command) {
        dbg!(command);
        let mut temp_boxes: VecDeque<char> = VecDeque::new();
        for _ in 0..command.num_of_boxes {
            let temp_box = self.stacks[(command.from_idx-1) as usize].0.pop_front().unwrap();
            temp_boxes.push_front(temp_box);
            // self.stacks[(command.to_idx - 1) as usize].0.push_front(temp_box);
        }
        println!("temp_boxes: {:?}", temp_boxes);
        for idx in 0..temp_boxes.len() {
            println!("idx: {}", idx);
            self.stacks[(command.to_idx - 1) as usize].0.push_front(temp_boxes.pop_front().unwrap());
        }
        dbg!(self);
    }

    fn run_commands(&mut self, commands: Vec<&str>, part: u8) {
        for command_str in commands {
            match part {
                1 => self.run_command(command_str.parse().unwrap()),
                2 => self.run_command_2(command_str.parse().unwrap()),
                _ => panic!("invalid part number"),
            }
        }
    }

    fn top_boxes(&self) -> String {
        let mut top_boxes: Vec<char> = Vec::new();
        for stack in &self.stacks {
            top_boxes.push(stack.0[0]);
        }
        top_boxes.iter().collect()
    }
}

#[derive(Debug, Clone, Copy)]
struct Command {
    num_of_boxes: u32,
    from_idx: u32,
    to_idx: u32,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        let mut idx = 5;
        let mut box_chars: Vec<char> = Vec::new();
        while chars[idx].is_ascii_digit() {
            box_chars.push(chars[idx]);
            idx += 1;
        }
        while !chars[idx].is_ascii_digit() {
            idx += 1;
        }
        let mut from_chars: Vec<char> = Vec::new();
        while chars[idx].is_ascii_digit() {
            from_chars.push(chars[idx]);
            idx += 1;
        }
        while !chars[idx].is_ascii_digit() {
            idx += 1;
        }
        let to_chars: Vec<char> = chars[idx..s.len()].to_vec();

        let num_of_boxes = box_chars.into_iter().collect::<String>().parse::<u32>().unwrap();
        let from_idx = from_chars.into_iter().collect::<String>().parse::<u32>().unwrap();
        let to_idx = to_chars.into_iter().collect::<String>().parse::<u32>().unwrap();
        Ok(Self{num_of_boxes, from_idx, to_idx})
    }
}

#[derive(Debug, Default)]
struct BoxStack(VecDeque<char>);

pub fn process_part1(input: &str) -> String {
    let mut width = 0;
    let mut lines_iter = input.lines();
    let boxes: Vec<SupplyBox> = lines_iter
        .by_ref()
        .take_while(|line| !line.contains('1'))
        .map(|line| {
            let mut total_idx = 0;
            let mut idx = 0;
            let mut acc: Vec<char> = Vec::new();
            let mut boxes: Vec<SupplyBox> = Vec::new();
            line.chars().for_each(|char| {
                if idx < 3 && total_idx < line.len()-1 {
                    acc.push(char);
                    idx += 1;
                } else if  total_idx == line.len()-1{
                    acc.push(char);
                    boxes.push(acc.iter().collect::<String>().parse::<SupplyBox>().unwrap());
                } else {
                    boxes.push(acc.iter().collect::<String>().parse::<SupplyBox>().unwrap());
                    acc.clear();
                    idx = 0;
                }
                total_idx += 1;
            });
            width = boxes.len();
            boxes    
        })
        .flatten()
        .collect();
    let mut warehouse = Warehouse::new(boxes, width);   
    lines_iter.next().unwrap();
    let commands: Vec<&str> = lines_iter.map(|line| line.clone()).collect();
    warehouse.run_commands(commands, 1);
    warehouse.top_boxes()
}

pub fn process_part2(input: &str) -> String {
    let mut width = 0;
    let mut lines_iter = input.lines();
    let boxes: Vec<SupplyBox> = lines_iter
        .by_ref()
        .take_while(|line| !line.contains('1'))
        .map(|line| {
            let mut total_idx = 0;
            let mut idx = 0;
            let mut acc: Vec<char> = Vec::new();
            let mut boxes: Vec<SupplyBox> = Vec::new();
            line.chars().for_each(|char| {
                if idx < 3 && total_idx < line.len()-1 {
                    acc.push(char);
                    idx += 1;
                } else if  total_idx == line.len()-1{
                    acc.push(char);
                    boxes.push(acc.iter().collect::<String>().parse::<SupplyBox>().unwrap());
                } else {
                    boxes.push(acc.iter().collect::<String>().parse::<SupplyBox>().unwrap());
                    acc.clear();
                    idx = 0;
                }
                total_idx += 1;
            });
            width = boxes.len();
            boxes    
        })
        .flatten()
        .collect();
    let mut warehouse = Warehouse::new(boxes, width);   
    lines_iter.next().unwrap();
    let commands: Vec<&str> = lines_iter.map(|line| line.clone()).collect();
    warehouse.run_commands(commands, 2);
    warehouse.top_boxes()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part_one() {
        let result = process_part1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part_two() {
        let result = process_part2(INPUT);
        assert_eq!(result, "MCD")
    }
}
