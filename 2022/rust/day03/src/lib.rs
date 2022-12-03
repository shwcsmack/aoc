//cargo run --bin part-1
//cargo run --bin part-2
//cargo watch -x check -x test

use std::str::FromStr;

struct ElfCommune {
    groups: Vec<ElfGroup>,
}

impl ElfCommune {
    fn total(&self) -> u32 {
        self.groups.iter().map(|group| group.total() as u32).sum()
    }
}

impl FromStr for ElfCommune {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let chunks: Vec<String> = lines.chunks(3).into_iter().map(|chunk| format!("{}\n{}\n{}", chunk[0], chunk[1], chunk[2])).collect();
        let groups: Vec<ElfGroup> = chunks.iter().map(|group| group.parse().unwrap()).collect();
        Ok(Self{groups})
    }
}
struct ElfGroup {
    elfs: Vec<Elf>,
}

impl ElfGroup {
    fn total(&self) -> u8 {
        let first_elf_items = &self.elfs.first().unwrap().0.items;
        let matches: Vec<&Item> = first_elf_items.iter().filter(|item| self.elfs[1].contains(item) && self.elfs[2].contains(item)).collect();
        matches[0].clone().try_into().unwrap()

    }
}

impl FromStr for ElfGroup {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.lines().count() == 3 {
            let elfs: Vec<Elf> = s.lines().map(|line| line.parse().unwrap()).collect();
            Ok(ElfGroup{ elfs})
        } else {
            Err("Incorrect number of lines".to_string())
        }
    }
}
struct Elf(Rucksack);

impl Elf {
    fn contains(&self, item: &Item) -> bool {
        self.0.contains(item)
    }
}

impl FromStr for Elf {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ruck: Rucksack = s.parse()?;
        Ok(Elf(ruck))
    }
}

#[derive(Debug)]
struct Rucksack {
    items: Vec<Item>,
}

impl Rucksack {
    fn new(str: &str) -> Self {
        Self { items: str.trim().chars().map(|char| Item(char)).collect() }
    }

    fn contains(&self, item: &Item) -> bool {
        self.items.contains(item)
    }

    fn total(&self) -> Result<i32, String> {
        let items = &self.items;
        let (left, right) = items.split_at(items.len()/2);
        let repeat = left.iter().find(|item| right.contains(item)).unwrap().to_owned();
        let digit: u8 = repeat.try_into()?;
        Ok(digit.into())
    }
}

impl FromStr for Rucksack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
struct Item(char);

impl TryInto<u8> for Item {
    type Error = String;

    fn try_into(self) -> Result<u8, Self::Error> {
        let matches = ('a'..='z').chain('A'..='Z')
        .zip(1..=52)
        .filter(|(char, _)| *char==self.0)
        .map(|(_, value)| value)
        .collect::<Vec<u8>>();
        if  matches.len() == 1 {
            Ok(matches[0])
        } else {
            Err(format!("Couldnt parse character: {}", self.0))
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let rucksacks = input.lines().map(|line| line.parse().unwrap()).collect::<Vec<Rucksack>>();
    rucksacks.iter().map(|ruck| ruck.total().unwrap()).sum::<i32>().to_string()
}

pub fn process_part2(input: &str) -> String {
    let commune: ElfCommune = input.parse().unwrap();

    commune.total().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_one() {
        let result = process_part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part_two() {
        let result = process_part2(INPUT);
        assert_eq!(result, "70")
    }
}
