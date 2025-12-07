use std::{process, time::Instant};
use day_06::{parse_input_one, parse_input_two, run};

fn main() {

    let now = Instant::now();
    
    let file_name = "input.txt";

    let (numbers, operations) = match parse_input_two(file_name) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        },
    }; 

    let result = match run(numbers, operations) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        },
    };

    println!("{result}");

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);

}


