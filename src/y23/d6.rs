use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::bytes::complete::{is_a, tag};
use nom::{Err, IResult};

#[path = "../solve.rs"] pub mod solve;


const TAGS:[&str; 2] =
            ["Time:",
             "Distance:"];

pub struct Dx {
    state: usize,
    time: Vec<u64>,
    dist: Vec<u64>,
}

impl Dx {

}

fn parse<'a>(inp: &'a str, pattern: &str) -> IResult<&'a str, Vec<u64>> {
    let mut res = vec![];
    let mut o_str: &str = "";
    let (seed_str, _t) = tag(pattern)(inp)?;
    let (seed_str, _t) = is_a(" ")(seed_str)?;
    if seed_str.len() > 0 {
        (o_str, res) = separated_list0(is_a(" "), digit1)(seed_str)?;
    };
    let res1 = res.join("").parse::<u64>().unwrap();

    let res2 = vec![res1];

    return Ok((o_str, res2));
}

fn n_opt(t: u64, d: u64) -> u64 {

    let x1 = 0.5 * (t as f64 - f64::sqrt((t as f64 * t as f64 - 4.0 * d as f64).into()));
    let x2 = 0.5 * (t as f64 + f64::sqrt((t as f64 * t as f64 - 4.0 * d as f64).into()));

    let a = (x1 * 1.00000001).ceil() as u64;
    let b = (x2 / 1.00000001).trunc() as u64;

    println!("{} - {} - {} - {}", x1, a, b, x2);

    return b-a+1;
}

impl solve::Solve for Dx {
    fn new() -> Dx
    {
        Dx {
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
