use nom::character::complete::{digit1, space1};
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::IResult;
use std::vec::Vec;
#[path = "../solve.rs"] pub mod solve;

pub struct Dx {
    sum: u64,
}

fn get_sample<'a>(inp: &'a str) -> IResult<&'a str, Vec<u64>> {
    let parse = map_res(digit1, |s: &str| s.parse::<u64>());
    separated_list0(space1, parse)(inp)
}

fn check(inp: &Vec<u64>) -> u64 {
    let dir: i64 = inp[0] as i64 - inp[1] as i64;
    if dir == 0 { return 0 }
    
    if dir > 0 {
        let mut it = inp.iter();
        let mut a = it.next().unwrap();
        for b in it {
            if (a == b) || (a < b) || ((a - b) > 3) { return 0 };

            a = b;
        }
    }

    if dir < 0 {
        let mut it = inp.iter();
        let mut a = it.next().unwrap();
        for b in it {
            if (a == b) || (a > b) || ((b - a) > 3) { return 0 };
            a = b;
        }
    }

    1
}

fn process(inp: &Vec<u64>) -> u64 {
    for a in 0..inp.len() {
        let mut vec = inp.clone();
        vec.remove(a);
        if check(&vec) == 1 { return 1 };
    }
    0
}

impl solve::Solve for Dx {
    fn new() -> Dx 
    {
        Dx { sum: 0 }
    }

    fn process(&mut self, inp: &String)
    {
        match get_sample(inp.as_str()) {
            Ok((_, sample)) => {
                let safe = process(&sample);
                if safe == 1 {self.sum+=1};
                //println!("{:?} -> {}", sample, safe);
            },
            Err(_) => panic!("Parsing error!"),
        }
    }

    fn result(&mut self) -> String
    {
        return self.sum.to_string();
    }
}
