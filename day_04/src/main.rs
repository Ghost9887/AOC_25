use std::{process, time::Instant};
use day_04::{parse_input, run};

fn main() {
    
    let now = Instant::now();

    let file_name = "input.txt";
    
    let map = match parse_input(file_name) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        },
    };

    let result = match run(map) {
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

