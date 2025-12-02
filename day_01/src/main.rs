use std::{fs::read_to_string, process, error::Error, time::Instant};
use day_01::parse_instruction;

fn main() {
    
    let now = Instant::now();
    
    let file_name = "input.txt";
    if let Err(e) = run(file_name) {
        eprintln!("Error: {e}");
        process::exit(1);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn run(file_name: &str) -> Result<(), Box<dyn Error>>{
    let mut instructions: Vec<&str> = Vec::new();
    
    let binding = read_to_string(file_name)?;
    for line in binding.lines() {
        instructions.push(line);
    }

    let mut zero_count = 0;
    let mut input_count = 50;
    
    for instruction in instructions {
        //println!("{input_count}");
        let mut temp = parse_instruction(&instruction);
        loop {
            if temp > 0 {
                input_count += 1;
                temp -= 1;
                if input_count > 99 {
                    input_count = 0;
                }
                if input_count == 0 {
                    zero_count += 1;
                }
                if temp <= 0 {
                    break;
                }
            } 
            else{
                input_count -= 1; 
                temp += 1;
                if input_count < 0 {
                    input_count = 99;
                }
                if input_count == 0 {
                    zero_count += 1;
                }
                if temp >= 0 {
                    break;
                }
            }
        }
        
    }
    println!("Result: {zero_count}");
    Ok(())
}

