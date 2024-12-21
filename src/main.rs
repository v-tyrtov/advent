use std::io::{self, BufRead};
mod y24;
use y24::Problem;
use y24::Solve;

fn main() {

    let mut task = Problem::new();

    for ln in io::stdin().lock().lines() {
        match ln {
            Err(_) => break,
            Ok(s) => task.process(&s),
        }
    }

    println!("Result {}", task.result());

}
