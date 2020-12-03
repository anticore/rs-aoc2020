use crate::common::read_lines;

pub fn slope(lines: &Vec<String>, right: usize, down: usize) -> i128 {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;

    for line in lines.iter() {
        let len = line.trim().chars().count();

        if y != 0 && y % down == 0 {
            x = (x + right) % len;

            if line.chars().nth(x).unwrap() == '#' {
                count += 1;
            }
        }

        y += 1;
    }

    println!("result {}", count);

    return count;
}

pub fn run_ex_3_part_1() -> () {
    println!("Running exercise 3 part 1");

    let mut count = 0;

    if let Ok(lines) = read_lines("./ex3.input.txt") {
        count = slope(&lines, 3, 1);
    }

    println!("Result {}", count);
}   

pub fn run_ex_3_part_2() -> () {
    println!("Running exercise 3 part 2");

    let mut res: i128 = 1;

    if let Ok(lines) = read_lines("./ex3.input.txt") {
        res *= slope(&lines, 1, 1);
        res *= slope(&lines, 3, 1);
        res *= slope(&lines, 5, 1);
        res *= slope(&lines, 7, 1);
        res *= slope(&lines, 1, 2);
    }

    println!("Result {}", res);
}   