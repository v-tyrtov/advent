use std::io::{self, BufRead};
mod y23;
use y23::Problem;
use y23::Solve;

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
