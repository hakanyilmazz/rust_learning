#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {}

fn some_math_ops() {
    let num_1: f32 = 1.111111111111111;
    println!("f32 : {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64 : {}", num_2 + 0.111111111111111);
    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 - num_4);
    num_3 += 1;
}

fn max_min_values_of_number_data_types() {
    println!("Max u32   : {}", u32::MAX);
    println!("Max u64   : {}", u64::MAX);
    println!("Max usize : {}", usize::MAX);
    println!("Max u128  : {}", u128::MAX);
    println!("Max f32   : {}", f32::MAX);
    println!("Max f64   : {}", f64::MAX);
}

fn some_data_types() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
    let is_true = true;
    let my_grade = 'A';
}

fn read_input_and_print_line() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't Receive Input!");

    println!("Hello {}, {}!", name.trim_end(), greeting);
}
