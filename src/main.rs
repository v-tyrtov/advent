use std::io::{self, BufRead};

mod d6;
use crate::d6::solve::Solve;

fn main() {

    let mut task = d6::D6::new();

    for ln in io::stdin().lock().lines() {
        match ln {
            Err(_) => break,
            Ok(s) => task.process(&s),
        }
    }

    println!("Result {}", task.result());

}
