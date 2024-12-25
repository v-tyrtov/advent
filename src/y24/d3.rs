use nom::character::complete::digit1;
use nom::bytes::complete::{tag,take_until};
use nom::combinator::map_res;
use nom::sequence::{delimited,separated_pair};
use nom::IResult;
use nom::Err;
use nom::error;
use nom::error::Error;
use std::cmp;
#[path = "../solve.rs"] pub mod solve;

pub struct Dx {
   // sum: u64,
    puzzle: String,
}

fn parse(inp: &str) -> IResult<&str, u64>
{
    return map_res(digit1, |s: &str| s.parse::<u64>())(inp);
}

fn get_mul<'a>(inp: &'a str) -> IResult<&'a str, (u64, u64)>
{
    delimited(tag("mul("),separated_pair(parse,tag(","),parse),tag(")"))(inp)
}


fn tags<'a>(inp: &'a str) -> IResult<&'a str, &str>
{
    let _do = take_until("do()")(inp);
    let _dont = take_until("don't()")(inp);
    let _mul = take_until("mul(")(inp);

    let _do_len = if let Ok((_str, _)) = _do { _str.len() } else {0};
    let mut _dont_len = if let Ok((_str, _)) = _dont { _str.len() } else {0};
    let mut _mut_len = if let Ok((_str, _)) = _mul { _str.len() } else {0};

    let _max = cmp::max(_do_len, cmp::max(_dont_len, _mut_len));

    if _max == _do_len {return _do};
    if _max == _dont_len {return _dont};

    _mul
}

fn sum_mul(inp: &str) -> u64
{
        let mut st = inp; // .as_str();

        let mut sum = 0;

        let mut enabled = true;

        while !st.is_empty() {

            match tags(st) {
               Ok((_str, _)) => {
                   let check_do = tag::<&str, &str, Error<&str>>("do()")(_str);
                   if check_do.is_ok() {
                        enabled = true;
                        let (_str, _) = check_do.unwrap();
                        st = _str;
                        //println!("DO  : {}", _str);
                        continue
                   }
                   if let Ok((_dstr, _)) = tag::<&str, &str, Error<&str>>("don't()")(_str) {
                        enabled = false;
                        st = _dstr;
                        //println!("DONT: {}", _str);
                        continue
                   }

                   // This should be 'mul('
                   match get_mul(_str) {
                     Ok((_str, (a,b))) => {
                        st = _str;
                        //println!("MUL : {} x {} - {}", a, b, _str);
                        if enabled { sum+=a*b; }
                     },
                     Err(Err::Error(error::Error {input: _str, code: _ } )) => {
                        st = _str;
                        //println!("ERR : {}", _str);
                     },
                     Err(_) => todo!(),
                   }

                }
                Err(_) => {
                    // No more tags in the string
                    st = "";
                }
            }
        }

    sum
}

impl solve::Solve for Dx {
    fn new() -> Dx
    {
        Dx { puzzle: String::new() }
    }

    fn process(&mut self, inp: &String)
    {
        self.puzzle = String::from(inp.as_str());
    }

    fn result(&mut self) -> String
    {
        let sum = sum_mul(self.puzzle.as_str());
        return sum.to_string();
    }
}
