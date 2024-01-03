use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::bytes::complete::{is_a, tag};
use nom::{Err, IResult};

#[path = "solve.rs"] pub mod solve;


const TAGS:[&str; 2] =
            ["Time:",
             "Distance:"];

pub struct D6 {
    state: usize,
    time: Vec<u32>,
    dist: Vec<u32>,
}

impl D6 {

}

fn parse<'a>(inp: &'a str, pattern: &str) -> IResult<&'a str, Vec<u32>> {
    let mut res = vec![];
    let mut o_str: &str = "";
    let parse = map_res(digit1, |s: &str| s.parse::<u32>());
    let (seed_str, _t) = tag(pattern)(inp)?;
    let (seed_str, _t) = is_a(" ")(seed_str)?;
    if seed_str.len() > 0 {
        (o_str, res) = separated_list0(is_a(" "), parse)(seed_str)?;
    };

    return Ok((o_str, res));
}

fn n_opt(t: u32, d: u32) -> u32 {

    let x1 = 0.5 * (t as f64 - f64::sqrt((t*t - 4*d).into()));
    let x2 = 0.5 * (t as f64 + f64::sqrt((t*t - 4*d).into()));

    let a = (x1 * 1.001).ceil() as u32;
    let b = (x2 / 1.001).trunc() as u32;

    println!("{} - {} - {} - {}", x1, a, b, x2);

    return b-a+1;
}

impl solve::Solve for D6 {
    fn new() -> D6
    {
        D6 {
            state: 0,
            time: vec![],
            dist: vec![]
        }
    }

    fn process(&mut self, inp: &String)
    {
        let mut _s = "";
        if !inp.is_empty() {
            let v;
            match parse(inp.as_str(), TAGS[self.state]) {
                Ok((_s, r)) => v = r,
                Err(Err::Error(s)) => panic!("Parsing Error for {}", s),
                Err(_) => panic!("Other parsing err!"),
            }

            match self.state {
                0 => self.time = v,
                1 => self.dist = v,
                _ => { println!("End") },
            }
            self.state += 1;
        }
    }

    fn result(&mut self) -> String
    {

        let mut res = 1;

        for a in 0..self.time.len() {
            res *= n_opt(self.time[a], self.dist[a]);
        }

        return res.to_string();
    }
}
