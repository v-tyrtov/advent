use std::io::{self, BufRead};

mod d4;
use crate::d4::solve::Solve;

fn main() {

    let mut task = d4::D4::new();

    for ln in io::stdin().lock().lines() {
        match ln {
            Err(_) => break,
            Ok(s) => task.process(&s),
        }
    }

    println!("Result {}", task.result());

}
