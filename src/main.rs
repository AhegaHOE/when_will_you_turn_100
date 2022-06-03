extern crate chrono;

use std::io;
use chrono::prelude::*;

fn main() {
    let name = input("What is your name?")
        .expect("Oops, something went wrong.");
    let age = input("What is your age?")
        .expect("Failed to parse your age")
        .parse::<i32>()
        .expect("Invalid Age, check if your age is a valid number");
    let year_when_hundred = 100 - age + Utc::now().year();
    println!("Hey, {} you are currently {} years old and you will turn 100 in the year {}.", name, age, year_when_hundred);
}

fn input(user_message: &str) -> io::Result<String> {
    println!("{}", user_message);
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}