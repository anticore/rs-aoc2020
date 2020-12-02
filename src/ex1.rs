use crate::common::read_lines;

pub fn run_ex_1_part_1() -> () {
    println!("Running exercise 1 part 1");

    let mut numbers: [i32; 200] = [0; 200];
    
    let mut i: usize = 0;
    if let Ok(lines) = read_lines("./ex1.input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                numbers[i] = ip.trim().parse().expect("Failed to convert");

                for num in numbers.iter() {
                    if num + numbers[i] == 2020 {
                        println!("Result {}", num * numbers[i]);
                        return;
                    }
                }

                i += 1;
            }
        }
    }
}   

pub fn run_ex_1_part_2() -> () {
    println!("Running exercise 1 part 2");

    let mut numbers: [i32; 200] = [0; 200];
    
    let mut k: usize = 0;
    if let Ok(lines) = read_lines("./ex1.input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let number: i32 = ip.trim().parse().expect("Failed to convert");
                numbers[k] = number;

                for i in 0..k {
                    for j in i..k {
                        if number + numbers[i] + numbers[j] == 2020 {
                            println!("Result {}", number * numbers[i] * numbers[j]);
                        }
                    }
                }

                k += 1;
            }
        }
    }
}   