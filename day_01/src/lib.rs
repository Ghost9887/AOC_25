
pub fn parse_instruction(instruction: &str) -> i32 {
    let mut number = String::new();
    let mut first_loop = false;
    for char in instruction.chars() {
        if !first_loop {
            if char == 'L' {
                number.push_str("-");
            }           
            first_loop = true;
            continue;
        }
        number.push_str(char.to_string().as_str());
    }
    number.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    
}
