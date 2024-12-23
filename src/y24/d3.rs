use nom::character::complete::digit1;
use nom::bytes::complete::{tag,take_until};
use nom::combinator::map_res;
use nom::sequence::{delimited,separated_pair};
use nom::IResult;
use nom::Err;
use nom::error;
use nom::error::{Error,ErrorKind};
#[path = "../solve.rs"] pub mod solve;

pub struct Dx {
    sum: u64,
}

fn parse(inp: &str) -> IResult<&str, u64>
{
    return map_res(digit1, |s: &str| s.parse::<u64>())(inp);
}

fn get_mul<'a>(inp: &'a str) -> IResult<&'a str, (u64, u64)> {
    let find_mul = take_until::<&str, &str, Error<&str> >("mul(")(inp);
    if find_mul.is_err() { return Err(Err::Error(Error::new("", ErrorKind::TakeUntil))) };
    delimited(tag("mul("),separated_pair(parse,tag(","),parse),tag(")"))(find_mul.unwrap().0)
}

impl solve::Solve for Dx {
    fn new() -> Dx 
    {
        Dx { sum: 0 }
    }

    fn process(&mut self, inp: &String)
    {
        let mut str = inp.as_str();

        while !str.is_empty() {
            match get_mul(str) {
                Ok((_str, (a,b))) => {
                    str = _str;
                    //println!("{} x {}", a, b);
                    self.sum+=a*b;
                },
                Err(Err::Error(error::Error {input: _str, code: _ } )) => {
                    str = _str; 
                },
                Err(_) => todo!(),
            }
        }
    }

    fn result(&mut self) -> String
    {
        return self.sum.to_string();
    }
}
