use std::{error::Error, fs};

pub fn parse_input_one(file_name: &str) -> Result<(Vec<Vec<i64>>, Vec<char>), Box<dyn Error>>{
    
    let mut numbers: Vec<Vec<i64>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();
   
    let content = fs::read_to_string(file_name)?;

    for line in content.lines() {
        if line.trim().chars().all(|c| c == '*' 
            || c == '+' || c.is_whitespace()) {

            operations = line.chars()
                .filter(|c| !c.is_whitespace())
                .collect(); 

            break;
        }
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        numbers.push(nums);
    }

    let number_col = numbers[0].len();
    let number_rows = numbers.len();

    let mut new_numbers = vec![Vec::with_capacity(number_rows); number_col];

    for i in 0..number_rows {
        for j in 0..number_col {
            new_numbers[j].push(numbers[i][j]);
        }
    }

    Ok((new_numbers, operations))
}

pub fn parse_input_two(file_name: &str) -> Result<(Vec<Vec<i64>>, Vec<char>), Box<dyn Error>> {
    let content = fs::read_to_string(file_name)?;

    let mut numbers: Vec<Vec<i64>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();

    

    Ok((numbers, operations))
}

pub fn run(numbers: Vec<Vec<i64>>, operations: Vec<char>) -> Result<i64, Box<dyn Error>> {
    let mut result = 0;
    
    for (i, number) in numbers.iter().enumerate() {
        let mut index = 0;
        let op = operations[i];
        let mut temp = 0;

        while index < number.len() {
            match op {
                '+' => {
                    if index < 1 {
                        temp = number[index];
                    }           
                    else {
                        temp = number[index] + temp;
                    }
                }
                '*' => {
                    if index < 1 {
                        temp = number[index];
                    }           
                    else {
                        temp = number[index] * temp;
                    }
                }
                _ => {
                    return Err("Invalid character".into());
                }
            }
            index += 1;
        }
        result += temp;
    }

    Ok(result)
}


#[cfg(test)]
mod tests {

    use super::*;
    use std::process;

    #[test]
    fn part_one() {
        let numbers: Vec<Vec<i64>> = vec![
            vec![123, 45, 6],
            vec![328, 64, 98],
            vec![51, 387, 215],
            vec![64, 23, 314],
        ];
        let operations: Vec<char> = vec![
            '*',
            '+',
            '*',
            '+',
        ];
        let result = match run(numbers, operations) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            }
        };
        assert_eq!(result, 4277556);
    }

    #[test]
    fn part_two() {
        let numbers: Vec<Vec<i64>> = vec![
            vec![4, 431, 623],
            vec![175, 581, 32],
            vec![8, 248, 369],
            vec![356, 24, 1],
        ];
        let operations: Vec<char> = vec![
            '+',
            '*',
            '+',
            '*',
        ];
        let result = match run(numbers, operations) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            }
        };
        assert_eq!(result, 3263827);
    }

}
