use std::str::FromStr;


#[derive(Debug, PartialEq, Eq)]
pub struct Matrix{
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Matrix {
    fn data_at(&self, x: usize, y: usize) -> Result<u8, String> {
        if x < self.width && y < self.height {
            Ok(self.data[(y*self.width + x)])
        } else if x >= self.width {
            Err(format!("Accessed index out of bounds. x:{} width:{}", x, self.width))
        } else if y >= self.height {
            Err(format!("Accessed index out of bounds. y:{} height:{}", y, self.height))
        } else {
            Err(format!("How did we get here. x:{} y:{}", x, y))
        }
    }

    pub fn row(&self, row: usize) -> Result<Vec<u8>, String> {
        if row < self.height {
            Ok(self.data[row*self.width..row*self.width+self.width].to_vec().clone())
        } else {
            Err(format!("Bad index. row:{} height:{}", row, self.height))
        }
    }

    pub fn column(&self, col: usize) -> Result<Vec<u8>, String> {
        let mut output:Vec<u8> = Vec::new();
        if col < self.width {
            for idx in 0..self.height {
                output.push(self.data_at(col, idx).unwrap());
            }
            Ok(output)
        } else {
            Err(format!("Bad index. col:{} width:{}", col, self.width))
        }
    }

    pub fn column_sum(&self, col: usize) -> Result<u64, String> {
        let mut sum: u64 = 0;
        if col < self.width {
            for idx in 0..self.height {
                sum += self.data_at(col, idx).unwrap() as u64;
            }
            Ok(sum)
        } else {
            Err(format!("Bad index. col:{} width:{}", col, self.width))
        }
    }

    pub fn most_common_bit_in_col(&self, col: usize) -> Result<u8, String> {
        if col < self.width {
            let sum = self.column_sum(col).unwrap();
            let half_height = self.height / 2;
            if sum > half_height as u64 {
                Ok(1)
            } else {
                Ok(0)
            }

        } else {
            Err(format!("Bad index. col:{} width:{}", col, self.width))
        }
    }

    pub fn gamma_rate_binary(&self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();
        for col in 0..self.width {
            output.push(self.most_common_bit_in_col(col).unwrap())
        }
        output
    }

   pub fn epsilon_rate_binary(&self) -> Vec<u8> {
        self.gamma_rate_binary().iter().map(|bit| {
            if *bit == 1 {
                0
            } else {
                1
            }
        }).collect()
   } 

    pub fn gamma_rate_decimal(&self) -> usize {
        Self::vec_u8_into_usize(self.gamma_rate_binary())
    }

    pub fn epsilon_rate_decimal(&self) -> usize {
        Self::vec_u8_into_usize(self.epsilon_rate_binary())
    }

    fn vec_u8_into_usize(input: Vec<u8>) -> usize {
        input
            .iter()
            .rev()
            .enumerate()
            .inspect(|(idx, bit)| println!("idx:{} bit:{}", idx, bit))
            .map(|(idx, bit)| {
                if idx == 0 {
                    *bit as usize
                } else if idx == 1 {
                    (*bit as usize)*2
                } else {
                    (*bit as usize)*(2 as usize).pow(idx as u32)
                }
            })
            .inspect(|num| println!("converted to: {}", num)) 
            .sum()
    }

    pub fn power_consumption(&self) -> usize {
        self.gamma_rate_decimal() * self.epsilon_rate_decimal()
    }
} 

impl FromStr for Matrix {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines_iter = s.lines().peekable();
        let lines_count = lines_iter.clone().count();
        let num_of_bits = lines_iter.peek().unwrap().chars().count();
        let flat_input: Vec<u8> = lines_iter
            .map(|line| line.trim())
            .flat_map(|s| s.chars())
            .filter_map(|char| char.to_digit(2))
            .filter_map(|n| n.try_into().ok())
            .collect();
        // dbg!(flat_input);
        let matrix = Matrix{data: flat_input, width: num_of_bits, height: lines_count};
        Ok(matrix)
    }
}


#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    const SMALL_INPUT: &str = "00100
        11110
        10110";

    const INPUT: &str = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";

    fn make_sut() -> Matrix {
        INPUT.parse::<Matrix>().unwrap()
    }

    #[test]
    fn parse_works() {
        let input = "00100
        11110
        10110";
        let sut = Matrix{data: vec![0,0,1,0,0,1,1,1,1,0,1,0,1,1,0], width: 5, height: 3};
        let result = input.parse::<Matrix>().unwrap();
        assert_eq!(sut, result)
    }

    #[test]
    fn data_at_works() {
        let sut = make_sut();

        assert_eq!(sut.data_at(0, 0), Ok(0));
        assert_eq!(sut.data_at(4, 0), Ok(0));
        assert_eq!(sut.data_at(2, 4), Ok(1));
        assert_eq!(sut.data_at(4, 11), Ok(0));
    }
    
    #[test]
    fn row_works() {
        let sut = make_sut();

        assert_eq!(sut.row(0), Ok(vec![0,0,1,0,0]));
        assert_eq!(sut.row(11), Ok(vec![0,1,0,1,0]));
    }

    #[test]
    fn column_works() {
        let sut: Matrix = SMALL_INPUT.parse().unwrap();

        assert_eq!(sut.column(0), Ok(vec![0,1,1]));
    }

    #[test]
    fn column_sum_works() {
        let sut: Matrix = SMALL_INPUT.parse().unwrap();

        assert_eq!(sut.column_sum(0), Ok(2));
        assert_eq!(sut.column_sum(2), Ok(3));
    }

    #[test]
    fn most_common_bit_in_col_works() {
        let sut: Matrix = SMALL_INPUT.parse().unwrap();

        assert_eq!(sut.most_common_bit_in_col(0), Ok(1));
        assert_eq!(sut.most_common_bit_in_col(1), Ok(0));
        assert_eq!(sut.most_common_bit_in_col(2), Ok(1));
        assert_eq!(sut.most_common_bit_in_col(3), Ok(1));
        assert_eq!(sut.most_common_bit_in_col(4), Ok(0));
    }
     #[test]
     fn gamma_rate_binary_works() {
        let sut: Matrix = SMALL_INPUT.parse().unwrap();

        assert_eq!(sut.gamma_rate_binary(), vec![1,0,1,1,0]);

        let sut = make_sut();

        assert_eq!(sut.gamma_rate_binary(), vec![1,0,1,1,0]);
     }

     #[test]
     fn gamma_rate_decimal_works() {
        let sut: Matrix = SMALL_INPUT.parse().unwrap();

        assert_eq!(sut.gamma_rate_decimal(), 16+4+2);

        let sut = make_sut();

        assert_eq!(sut.gamma_rate_decimal(), 22);
     }

     #[test]
     fn epsilon_rate_binary_works() {
        let sut: Matrix = SMALL_INPUT.parse().unwrap();

        assert_eq!(sut.epsilon_rate_binary(), vec![0,1,0,0,1]);

        let sut = make_sut();

        assert_eq!(sut.epsilon_rate_binary(), vec![0,1,0,0,1]);
     }

     #[test]
     fn epsilon_rate_decimal_works() {
        let sut: Matrix = SMALL_INPUT.parse().unwrap();

        assert_eq!(sut.epsilon_rate_decimal(), 9);

        let sut = make_sut();

        assert_eq!(sut.epsilon_rate_decimal(), 9);
     }

     #[test]
     fn power_consumption_works() {
        let sut = make_sut();

        assert_eq!(sut.power_consumption(), 198);
     }
}