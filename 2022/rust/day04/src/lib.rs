//cargo run --bin part-1
//cargo run --bin part-2
//cargo watch -x check -x test

use std::{str::FromStr, num::ParseIntError};

#[derive(Debug, PartialEq, Eq)]
struct ElfPair(ElfRange, ElfRange);

impl ElfPair {
    fn fully_overlaps(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    fn overlaps(&self) -> bool {
        self.0.intersects(&self.1)
    }
}

impl FromStr for ElfPair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').map(|str| str.parse().unwrap()).collect::<Vec<ElfRange>>();
        if parts.len() == 2{
            Ok(ElfPair(parts[0], parts[1]))
        } else {
            Err(format!("Not a pair: {}", s))
        }

    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ElfRange {
    min: i32,
    max: i32,
}

impl ElfRange {
    fn contains(&self, other: &ElfRange) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn intersects(&self, other: &ElfRange) -> bool {
        self.min <= other.max && other.min <= self.max
    }
}

impl FromStr for ElfRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split('-')
            .map(|str| str.parse::<i32>())
            .collect::<Result<Vec<i32>, ParseIntError>>();
        
        match parts {
            Ok(nums) => {
                if nums.len() == 2 {
                    Ok(Self{ min: nums[0], max: nums[1]})
                } else {
                    Err(format!("Couldnt parse(not the right number of items): {}", s))
                }
            },
            Err(e) => Err(format!("Couldnt parse into i32: {}", e)),
        }
    }
}


pub fn process_part1(input: &str) -> String {
    input
    .lines()
    .map(|line| line.trim())
    .filter_map(|line| line.parse::<ElfPair>().ok())
    .filter(|elfpair| elfpair.fully_overlaps())
    .count()
    .to_string()
}

pub fn process_part2(input: &str) -> String {
    input
    .lines()
    .map(|line| line.trim())
    .filter_map(|str| str.parse::<ElfPair>().ok())
    .filter(|elfpair| elfpair.overlaps())
    .count()
    .to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";

    #[test]
    fn part_one() {
        let result = process_part1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn part_two() {
        let result = process_part2(INPUT);
        assert_eq!(result, "4")
    }

    #[test]
    fn parse_elf_pair() {
        let sut = ElfPair(ElfRange { min: 1, max: 5 }, ElfRange { min: 7, max: 13 });
        let str = "1-5,7-13";
        assert_eq!(str.parse::<ElfPair>().unwrap(), sut)
    }

    #[test]
    fn parse_elf_range() {
        let sut = ElfRange{ min: 1, max: 15};
        let str = "1-15";
        assert_eq!(str.parse::<ElfRange>().unwrap(), sut)
    }
}
