use std::{process, time::Instant};
use day_05::{parse_foods, parse_ranges, run_part_one, run_part_two};

fn main() {
    
    let now = Instant::now();

    let file_name = "input.txt";
    
    let ranges: Vec<Vec<i64>> = match parse_ranges(file_name) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        },
    };

    let foods: Vec<i64> = match parse_foods(file_name) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        },
    };

    
    let result = match run_part_one(ranges, foods) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        },
    };
    
    /*
    let result = match run_part_two(ranges) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        },
    };
    */
    
    println!("Result: {result}");

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
}
