use std::{fs, error::Error};

pub fn parse_input(file_name: &str) -> Result<Vec<Vec<i8>>, Box<dyn Error>>{

    let mut banks: Vec<Vec<i8>> = Vec::new();
    let content = fs::read_to_string(file_name)?;   
    
    for line in content.lines() {
        let mut bank: Vec<i8> = Vec::new();
        for char in line.chars() {
            bank.push(char.to_string().parse::<i8>()?);
        }
        banks.push(bank);
    }

    Ok(banks)
}

pub fn run(banks: Vec<Vec<i8>>, count: usize) -> Result<i64, Box<dyn Error>> {
    let mut result: i64 = 0;
    
    for bank in banks {
        let mut numbers: Vec<i8> = Vec::new();
        let mut start_idx = 0;

        for numbers_left in (0..count).rev() {
            let mut highest_number = 0;
            let mut highest_index = 0;
            for (i, &value) in bank[start_idx..bank.len() - numbers_left].iter().enumerate() {
                if value > highest_number {
                    highest_number = value;
                    highest_index = i;
                }
            }
            numbers.push(highest_number);
            start_idx += highest_index + 1;
        }
        let temp = build_result(&numbers);
        result += temp; 
    }
    Ok(result)
}

fn build_result(numbers: &Vec<i8>) -> i64 {
    let mut index: usize = 0;
    let mut mult: i64 = (numbers.len() - 1) as i64; 
    let mut result: i64 = 0;

    while index < numbers.len() - 1 {
        result += (numbers[index] as i64) * (10_i64.pow(mult as u32));
        mult -= 1;
        index += 1;
    }

    result += numbers[numbers.len() - 1] as i64;
    result
}

#[cfg(test)]
mod tests{

    use super::*;
    use std::process;

    #[test]
    fn test_run_part_one(){
        let banks = vec![
            vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1], //98
            vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9], //89
            vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8], //78
            vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1], //92
        ];
        let result = match run(banks, 2) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            }
        };
        assert_eq!(result, 357);
    }
    
    #[test]
    fn test_run_part_two(){
        let banks = vec![
            vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1], //987654321111
            vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9], //811111111119
            vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8], //434234234278
            vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1], //888911112111
        ];
        let result = match run(banks, 12) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            }
        };
        assert_eq!(result, 3121910778619);
    }
}

