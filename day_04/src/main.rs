use std::{process, time::Instant};
use day_04::{parse_input, run_part_one, run_part_two};

fn main() {
    
    let now = Instant::now();

    let file_name = "input.txt";
    
    let mut map = match parse_input(file_name) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        },
    };

    let result = match run_part_one(&map) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            },
    };
    
    /*
    let mut result = 0;
    loop {
        let new_result = match run_part_two(&mut map) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            },
        };
        if new_result < 1 {
            break;
        }
        else {
            result += new_result;
        }
    }
    */

    println!("{result}");
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
}

