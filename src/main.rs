use std::io::{self, BufRead};

mod d3;
use crate::d3::solve::Solve;

fn main() {

    let mut task = d3::D3::new();

    for ln in io::stdin().lock().lines() {
        match ln {
            Err(_) => break,
            Ok(s) => task.process(&s),
        }
    }

    println!("Result {}", task.result());

}
