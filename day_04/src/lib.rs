use std::{fs, error::Error};

pub fn parse_input(file_name: &str) -> Result<Vec<Vec<i8>>, Box<dyn Error>> {
    let mut map: Vec<Vec<i8>> = Vec::new();
    let content = fs::read_to_string(file_name)?;
    for line in content.lines() {
        let mut map_line: Vec<i8> = Vec::new();
        for char in line.chars() {
            match char {
                '@' => {
                    map_line.push(1);
                },
                '.' => {
                    map_line.push(0);
                },
                _ => {
                    return Err("Invalid input!".into());
                }
            }
        }
        map.push(map_line);
    }
    Ok(map)
}

pub fn check_neighbours(j: isize, i: isize, map: &Vec<Vec<i8>>) -> bool {
    let mut found = 0;
    let neighbours = [
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];
    for (dj, di) in neighbours {
        let nj = j + dj;
        let ni = i + di;

        if nj >= 0 && ni >= 0 && 
           (nj as usize) < map.len() &&
           (ni as usize) < map[0].len() 
        {
            if map[nj as usize][ni as usize] == 1 {
                found += 1;
            }
        }
    }
    found < 4
}

pub fn run_part_one(map: &Vec<Vec<i8>>) -> Result<u32, Box<dyn Error>> {
    let mut result: u32 = 0;
    for j in 0..map.len() {
        for i in 0..map[j].len() {
            match map[j][i] {
                0 => continue,
                1 => {
                    let valid = check_neighbours(j as isize, i as isize, &map);
                    if valid {
                        result += 1;
                    }
                }
                _ => return Err("Invalid input".into()),
            }
        }
    }
    Ok(result)   
}

pub fn run_part_two(map: &mut Vec<Vec<i8>>) -> Result<u32, Box<dyn Error>> {
    let mut new_map: Vec<Vec<i8>> = map.clone();
    let mut result = 0;
    for j in 0..map.len() {
        for i in 0..map[j].len() {
            match map[j][i] {
                0 => continue,
                1 => {
                    let valid = check_neighbours(j as isize, i as isize, &map);
                    if valid {
                        new_map[j][i] = 0;
                        result += 1;
                    }
                }
                _ => return Err("Invalid input".into()),
            }
        }
    }
    *map = new_map.clone();
    Ok(result)
} 

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn part_one() {
        let map = vec![
            vec![0,0,1,1,0,1,1,1,1,0],
            vec![1,1,1,0,1,0,1,0,1,1],
            vec![1,1,1,1,1,0,1,0,1,1],
            vec![1,0,1,1,1,1,0,0,1,0],
            vec![1,1,0,1,1,1,1,0,1,1],
            vec![0,1,1,1,1,1,1,1,0,1],
            vec![0,1,0,1,0,1,0,1,1,1],
            vec![1,0,1,1,1,0,1,1,1,1],
            vec![0,1,1,1,1,1,1,1,1,0],
            vec![1,0,1,0,1,1,1,0,1,0],
        ];
        let result = run_part_one(&map).unwrap();
        println!("Result: {result}");
        assert_eq!(result, 13);
    }

    #[test]
    fn part_two() {
        let mut map = vec![
            vec![0,0,1,1,0,1,1,1,1,0],
            vec![1,1,1,0,1,0,1,0,1,1],
            vec![1,1,1,1,1,0,1,0,1,1],
            vec![1,0,1,1,1,1,0,0,1,0],
            vec![1,1,0,1,1,1,1,0,1,1],
            vec![0,1,1,1,1,1,1,1,0,1],
            vec![0,1,0,1,0,1,0,1,1,1],
            vec![1,0,1,1,1,0,1,1,1,1],
            vec![0,1,1,1,1,1,1,1,1,0],
            vec![1,0,1,0,1,1,1,0,1,0],
        ];
        let mut new_result = 0;
        loop {
            let result = run_part_two(&mut map).unwrap();
            if result < 1 {
                break;
            }else {
                new_result += result;
            }
        }
        assert_eq!(new_result, 43);
    }

}

