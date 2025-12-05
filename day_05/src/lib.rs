use std::{error::Error, fs};

pub fn parse_ranges(file_name: &str) -> Result<Vec<Vec<i64>>, Box<dyn Error>> {
    let mut ranges: Vec<Vec<i64>> = Vec::new();

    let content = fs::read_to_string(file_name)?;

    let mut start_range = String::new();
    let mut end_range = String::new();
    for line in content.lines() {

        let mut switch = false;
        let mut range: Vec<i64> = Vec::new();

        if line == "" {
            break;
        }
        for char in line.chars() {
            if switch && char.is_numeric(){
                end_range.push(char);
            }
            else if char.is_numeric() {
                start_range.push(char);
            }
            else {
                switch = true;
                continue;
            }
        }
        range.push(start_range.parse::<i64>()?);
        range.push(end_range.parse::<i64>()?);
        ranges.push(range);
        start_range.clear();
        end_range.clear();
    }
    Ok(ranges)
}

pub fn parse_foods(file_name: &str) -> Result<Vec<i64>, Box<dyn Error>> {
    let mut foods: Vec<i64> = Vec::new();

    let content = fs::read_to_string(file_name)?;
    let mut start_parsing = false;
    let mut food_id = String::new();

    for line in content.lines() {
        if start_parsing {
            for char in line.chars() {
                food_id.push(char);
            }
            foods.push(food_id.parse::<i64>()?);
            food_id.clear();
        }
        if line == "" && !start_parsing{
            start_parsing = true;
            continue;
        }
    }

    Ok(foods)
}

pub fn run_part_one(ranges: Vec<Vec<i64>>, mut foods: Vec<i64>) -> Result<i64, Box<dyn Error>> {
    let mut result = 0;
    for range in ranges {
        for food in foods.iter_mut() {
            if *food != 0 && *food >= range[0] && *food <= range[1] {
                result += 1;
                *food = 0;
            }
        }
    }
    Ok(result)
}

pub fn run_part_two(mut ranges: Vec<Vec<i64>>) -> Result<i64, Box<dyn Error>> {
    let mut result: i64 = 0;
    let mut new_ranges: Vec<Vec<i64>> = Vec::new();

    sort_ranges(&mut ranges);

    new_ranges.push(vec![ranges[0][0], ranges[0][1]]);

    for i in 1..ranges.len() {
        let new_int: Vec<i64> = vec![new_ranges[new_ranges.len() - 1][0], new_ranges[new_ranges.len() - 1][1]];
        let org_int: Vec<i64> = vec![ranges[i][0], ranges[i][1]];

        if new_int[1] >= org_int[0] && new_int[1] <= org_int[1] {
            new_ranges.pop();
            new_ranges.push(vec![new_int[0], org_int[1]]);
        }
        else if new_int[0] <= org_int[0] && new_int[1] >= org_int[1] {
            continue;
        }
        else {
            new_ranges.push(vec![ranges[i][0], ranges[i][1]]);
        }
    }

    /*
    for range in new_ranges {
        println!("{:?}", range);
    }
    */

    for range in new_ranges {
        result += (range[1] - range[0]) + 1;
    }

    Ok(result)
}

//bubble sort
fn sort_ranges(ranges: &mut Vec<Vec<i64>>) {
    for i in 0..ranges.len(){
        for j in 0..ranges.len() {
            if i == j {
                continue;
            }
            if ranges[j][0] > ranges[i][0] {
                let temp1: i64 = ranges[j][0];
                let temp2: i64 = ranges[j][1];
                ranges[j][0] = ranges[i][0];
                ranges[j][1] = ranges[i][1];
                ranges[i][0] = temp1;
                ranges[i][1] = temp2;
            }
        }
    }
}

#[cfg(test)]
mod tests{

    use super::*;
    use std::process;

    #[test]
    fn part_one(){
        let ranges = vec![
            vec![3, 5],
            vec![10, 14],
            vec![16, 20],
            vec![12, 18],
        ];

        let foods = vec![1, 5, 8, 11, 17, 32];
        let result = match run_part_one(ranges, foods) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            },
        };
        println!("Result: {result}");
        assert_eq!(result, 3);
    }

    #[test]
    fn part_two() {
        let ranges = vec![
            vec![3, 5],
            vec![10, 14],
            vec![16, 20],
            vec![12, 18],
        ];
        
        let result = match run_part_two(ranges) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            },
        };
        
        assert_eq!(result, 14);
    }
}
