use std::{fs, error::Error};

pub fn parse_input(file_name: &str) -> Result<Vec<Vec<i64>>, Box<dyn Error>>{

    let mut banks: Vec<Vec<i64>> = Vec::new();
    let content = fs::read_to_string(file_name)?;   
    
    for line in content.lines() {
        let mut bank: Vec<i64> = Vec::new();
        for char in line.chars() {
            bank.push(char.to_string().parse::<i64>()?);
        }
        banks.push(bank);
    }

    Ok(banks)
}

pub fn run_part_one(banks: Vec<Vec<i64>>) -> Result<i64, Box<dyn Error>> {
    
    let mut result: i64 = 0;
    let mut first_number: i64;
    let mut second_number: i64;
    for bank in banks {
        first_number = bank[0];
        second_number = bank[1];
        let mut index = 0;
        while index < bank.len() {
            if bank[index] > first_number && index + 1 < bank.len() {
                first_number = bank[index];
                second_number = 0;
            }
            if index + 1 < bank.len() && bank[index + 1] > second_number {
                second_number = bank[index + 1];
            }   
            index += 1;
        }
        let temp = first_number * 10 + second_number;
        println!("{temp}");
        result += temp;
    }
    Ok(result)
}

pub fn run_part_two(banks: Vec<Vec<i64>>) -> Result<i64, Box<dyn Error>> {
    Ok(0)
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
        let result = match run_part_one(banks) {
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
        let result = match run_part_two(banks) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            }
        };
        assert_eq!(result, 3121910778619);
    }
}

