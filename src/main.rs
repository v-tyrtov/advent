use std::io::{self, BufRead};

mod d5;
use crate::d5::solve::Solve;

fn main() {

    let mut task = d5::D5::new();

    for ln in io::stdin().lock().lines() {
        match ln {
            Err(_) => break,
            Ok(s) => task.process(&s),
        }
    }

    println!("Result {}", task.result());

}
