use day_02::{parse_input, check_if_valid};
use std::{process, error::Error};

fn main() {
    let file_name = "input.txt";
    
    let ranges = match parse_input(file_name) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
    if let Err(e) = run(ranges) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run(ranges: Vec<Vec<i64>>) -> Result<(), Box<dyn Error>> {
    
    //println!("{:?}", ranges);

    let mut result = 0;

    for range in ranges {
        let start = range[0]; 
        let end = range[1];
        for number in start..=end {
            result += check_if_valid(number);
        }
    }

    println!("Result: {result}");
    Ok(())
}

