use nom::character::complete::{digit1, space1};
use nom::combinator::map_res;
use nom::sequence::separated_pair;
use nom::IResult;
use std::vec::Vec;
#[path = "../solve.rs"] pub mod solve;

pub struct Dx {
    l1: Vec<u64>,
    l2: Vec<u64>,
}

fn to_u64<'a>(inp: &'a str) -> IResult<&'a str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>())(inp)
}

impl solve::Solve for Dx {
    fn new() -> Dx 
    {
        Dx { l1: Vec::new(), l2: Vec::new() }
    }

    fn process(&mut self, inp: &String)
    {
        match separated_pair(to_u64, space1, to_u64)(inp.as_str()) {
            Ok((_, (n1, n2))) => {
                self.l1.push(n1);
                self.l2.push(n2);
            },
            Err(_) => panic!("Parsing error")
        }
    }

    fn result(&mut self) -> String
    {
       
        let mut sum = 0;
        self.l1.sort();
        self.l2.sort();
        for idx in 0..self.l1.len() {
            let v = (self.l1[idx] as i64 - self.l2[idx] as i64).abs();
            sum += v;
        }
        return sum.to_string();
    }
}
