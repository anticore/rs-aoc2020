use regex::Regex;
use crate::common::read_lines;

fn is_password_valid(s: &str, c: &str, min: i32, max: i32) -> bool {
    let mut count = 0;

    for sc in s.chars() {
        if sc.to_string() == c {
            count += 1;
        }
    }

    return (min <= count) && (count <= max);
}

fn is_password_valid_part_2(s: &str, c: &str, i1: usize, i2: usize) -> bool {
    println!("{} {} {} {}", s, c, i1, i2);

    let c1 = s.chars().nth(i1-1).unwrap().to_string();
    let c2 = s.chars().nth(i2-1).unwrap().to_string();

    return (c1 == c && c2 != c) || (c1 != c && c2 == c);
}

pub fn run_ex_2_part_1() -> () {
    println!("Running exercise 2 part 1");

    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

    let mut count = 0;

    if let Ok(lines) = read_lines("./ex2.input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                for cap in re.captures_iter(&ip) {
                    let s = &cap[4];
                    let c = &cap[3];
                    let min = cap[1].to_string().parse().unwrap();
                    let max = cap[2].to_string().parse().unwrap();

                    if is_password_valid(s, c, min, max) {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Result {}", count);
}   

pub fn run_ex_2_part_2() -> () {
    println!("Running exercise 2 part 1");

    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

    let mut count = 0;

    if let Ok(lines) = read_lines("./ex2.input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                for cap in re.captures_iter(&ip) {
                    let s = &cap[4];
                    let c = &cap[3];
                    let i1 = cap[1].to_string().parse().unwrap();
                    let i2 = cap[2].to_string().parse().unwrap();

                    if is_password_valid_part_2(s, c, i1, i2) {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Result {}", count);
}   