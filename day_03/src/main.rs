use std::{process, time::Instant};
use day_03::{parse_input, run_part_one};

fn main() {
    let now = Instant::now();

    let file_name = "input.txt";

    let banks = match parse_input(file_name) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };

    //println!("{:?}", banks);
    let result = match run_part_one(banks) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
    
    println!("{result}");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
