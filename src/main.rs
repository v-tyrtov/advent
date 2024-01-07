use std::io::{self, BufRead};

mod d7;
use crate::d7::solve::Solve;

fn main() {

    let mut task = d7::D7::new();

    for ln in io::stdin().lock().lines() {
        match ln {
            Err(_) => break,
            Ok(s) => task.process(&s),
        }
    }

    println!("Result {}", task.result());

}
