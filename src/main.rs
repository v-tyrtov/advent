use std::io::{self, BufRead};

mod d2;
use crate::d2::solve::Solve;

fn main() {
    println!("Hellool");

    let mut task = d2::D2::new();

    for ln in io::stdin().lock().lines() {
        match ln {
            Err(_) => break,
            Ok(s) => task.process(&s),
        }
    }

    println!("Result {}", task.result());

}
