use crate::common::read_lines;

fn get_row(s: &str) -> isize {
    let mut b = String::from("");
    let ss: &str = &s[..7];

    for c in ss.chars() {
        b.push_str(if c=='F' {"0"} else {"1"});
    }

    return isize::from_str_radix(&b, 2).unwrap();
}

fn get_col(s: &str) -> isize {
    let mut b = String::from("");
    let ss: &str = &s[7..];

    for c in ss.chars() {
        b.push_str(if c=='L' {"0"} else {"1"});
    }

    return isize::from_str_radix(&b, 2).unwrap();
}

fn get_id(s: &str) -> isize {
    let row = get_row(s);
    let col = get_col(s);
    return row * 8 + col;
}

pub fn run_ex_5_part_1() -> () {
    println!("Running exercise 5 part 1");

    let mut max = 0;

    if let Ok(lines) = read_lines("./ex5.input.txt") {
        for line in lines {
            let id = get_id(&line);

            if id > max { max = id; }
        }
    }

    println!("Result {}", max);
}  

pub fn run_ex_5_part_2() -> () {
    println!("Running exercise 5 part 2");

    let mut arr: [isize; 900] = [0; 900];

    if let Ok(lines) = read_lines("./ex5.input.txt") {
        for line in lines {
            let id = get_id(&line) as usize;

            arr[id] = 1;
        }
    }

    for (i, _) in arr.iter().enumerate() {
        if i > 1 && arr[i] == 0 && arr[i-1] == 1 && arr[i+1] == 1 { 
            println!("Result {}", i);
            break;
        } 
    }
}  