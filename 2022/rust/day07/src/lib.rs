//cargo run --bin part-1
//cargo run --bin part-2
//cargo watch -x check -x test

pub fn process_part1(input: &str) -> String {
    todo!();
}

pub fn process_part2(input: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let result = process_part1(INPUT);
        assert_eq!(result, "0");
    }

    #[ignore]
    #[test]
    fn part_two() {
        let result = process_part2(INPUT);
        assert_eq!(result, "0")
    }
}
