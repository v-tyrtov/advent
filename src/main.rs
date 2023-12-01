use std::io::{self, BufRead};

mod d1;
use crate::d1::solve::Solve;

fn main() {
    println!("Hellool");

    let mut task = d1::D1::new();

    for ln in io::stdin().lock().lines() {
        match ln {
            Err(_) => break,
            Ok(s) => task.process(&s),
        }
    }

    println!("Result {}", task.result());

}
