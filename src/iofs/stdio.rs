#![allow(unused)]

use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

pub fn run() {
    input_from_console();
}

fn input_from_console() {
    println!("what is you name");

    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";

    io::stdin()
        .read_line(&mut name)
        .expect("Didn't Receive Input");
    println!("Hello {}! {}", name.trim_end(), greeting);
}
