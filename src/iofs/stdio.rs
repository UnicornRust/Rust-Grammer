#![allow(unused)]

use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

pub fn run() {
    // input_from_console();
    //
    file_create();
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

fn file_create() {
    let path = "temp/lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file:: {:?}", error);
        }
    };
    write!(output, "just some\n Random words")
        .expect("Failed to write to file");
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File::create("temp/rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("temp/rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", error),
            },
            _ther_error => panic!("Problem opening file: {:?}", error),
        },
    };
}
