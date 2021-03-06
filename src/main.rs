use std::io;
mod common;
mod ex1;
use ex1::*;
mod ex2;
use ex2::*;
mod ex3;
use ex3::*;
mod ex4;
use ex4::*;
mod ex5;
use ex5::*;

fn main() {
    println!("AoC 2020 - Choose exercise to run");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    if choice.trim() == "11" {
        run_ex_1_part_1();
    } else if choice.trim() == "12" {
        run_ex_1_part_2();
    } else if choice.trim() == "21" {
        run_ex_2_part_1();
    } else if choice.trim() == "22" {
        run_ex_2_part_2();
    } else if choice.trim() == "31" {
        run_ex_3_part_1();
    } else if choice.trim() == "32" {
        run_ex_3_part_2();
    } else if choice.trim() == "41" {
        run_ex_4_part_1();
    } else if choice.trim() == "42" {
        run_ex_4_part_2();
    } else if choice.trim() == "51" {
        run_ex_5_part_1();
    } else if choice.trim() == "52" {
        run_ex_5_part_2();
    }
}
