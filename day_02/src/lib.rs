use std::{fs, error::Error};

pub fn parse_input(file_name: &str) -> Result<Vec<Vec<i64>>, Box<dyn Error>>{
    let mut ranges: Vec<Vec<i64>> = Vec::new();
    let content = fs::read_to_string(file_name)?; 
    let mut starting_range = String::new();
    let mut ending_range = String::new();
    let mut range: Vec<i64> = Vec::new();
    let mut switch = false;

    for char in content.chars() {
        if !switch {
            if char.is_numeric() {
                starting_range.push(char);
            }
            else if char == '-' {
                switch = true;
                continue;
            }
        }else {
            if char.is_numeric() {
                ending_range.push(char);
            }
            else {
                range.push(starting_range.parse::<i64>()?);
                range.push(ending_range.parse::<i64>()?);
                ranges.push(range);
                range = Vec::new();
                starting_range = String::new();
                ending_range = String::new();
                switch = false;
                continue;
            }
        }
    }
    Ok(ranges)
}

pub fn check_if_valid(number: i64) -> i64 {
    let string_number = number.to_string();

    if string_number[0..string_number.len() / 2] == string_number[string_number.len() / 2..string_number.len()] {
        return number;
    }

    let mut index = 1;
    let mut pattern = String::new();
    let mut found_pattern = false;

    while index < string_number.len() / 2 + 1 {
        if !found_pattern {
            pattern.push_str(&string_number[0..index]);
            if pattern == &string_number[index..index + pattern.len()] {
                found_pattern = true;
                continue;
            }
            pattern = String::new();
            index += 1;
        }
        else{
            if string_number.len() % pattern.len() != 0 {
                return 0;
            }
            else {
                index = pattern.len();
                while index < string_number.len() {
                    if pattern == &string_number[index..index + pattern.len()]{
                        index += pattern.len();
                    }
                    else{
                        return 0;
                    }
                }
            }
        }
    } 
    if found_pattern {
        number
    }else {
        0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_valid_input(){
        let numbers = vec![
            vec![11, 22], 
            vec![95, 115],
            vec![998, 1012],
            vec![1188511880, 1188511890],
            vec![222220, 222224],
            vec![1698522, 1698528],
            vec![446443, 446449],
            vec![38593856, 38593862],
            vec![565653, 565659],
            vec![824824821, 824824827],
            vec![2121212118, 2121212124],
        ];
        let mut result = 0;
        for element in numbers {
            let start = element[0];
            let end = element[1];
            for number in start..=end{
                let out = check_if_valid(number);
                if out > 0{
                    println!("{out}");
                    result += out; 
                }
            }
        }
        assert_eq!(result, 4174379265);
    }
}
