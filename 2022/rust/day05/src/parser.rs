/// so we need a VecDeque for each stack of boxes
/// we will try to use nom to parse the input
/// the lines for the box stacks are 3 chars (spaces or the box delimited by []) seperated by spaces and then a newline
/// do we just take 3 and split on the space or actually search for '[','letter',']'
/// then its a row of indexes, do we need this or just use the index on the vec of VecDeque
/// then the commmands are 'move' space 'count' space 'from' space 'idx' space 'to' space 'idx' newline
/// https://github.com/Geal/nom/blob/main/doc/making_a_new_parser_from_scratch.md

use nom::IResult;
use nom::bytes::complete::tag;

// pub fn length_value(input: &[u8]) -> IResult<&[u8],&[u8]> {
//     let (input, length) = be_u16(input)?;
//     take(length)(input)
// }

// pub fn box_line(input: &str) -> IResult<&[u8], &[u8]> {
//     let line_ending = tag("\r\n");

// }



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

    // #[test]
    // fn part_one() {
    //     let result = process_part1(INPUT);
    //     assert_eq!(result, "CMZ");
    // }
}