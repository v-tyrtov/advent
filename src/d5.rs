use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::bytes::complete::tag;
use nom::IResult;
use std::collections::HashMap;

#[path = "solve.rs"] pub mod solve;


const TAGS:[&str; 8] =
            ["seeds: ",
             "seed-to-soil map:",
             "soil-to-fertilizer map:",
             "fertilizer-to-water map:",
             "water-to-light map:",
             "light-to-temperature map:",
             "temperature-to-humidity map:",
             "humidity-to-location map:"];

pub struct D5 {
    state: usize,
    first_str: bool,
    seeds: Vec<u32>,
    mapped: HashMap<usize,u32>,
}

impl D5 {

}

fn parse<'a>(inp: &'a str, pattern: &str) -> IResult<&'a str, Vec<u32>> {
    let mut res = vec![];
    let mut o_str: &str = "";
    let parse = map_res(digit1, |s: &str| s.parse::<u32>());
    let (seed_str, _t) = tag(pattern)(inp)?;
    if seed_str.len() > 0 {
        (o_str, res) = separated_list0(tag(" "), parse)(seed_str)?;
    };

    return Ok((o_str, res));
}

impl solve::Solve for D5 {
    fn new() -> D5
    {
        D5 {
            state: 0,
            first_str: false,
            seeds: vec![],
            mapped: HashMap::new()
        }
    }

    fn process(&mut self, inp: &String)
    {
        let mut _s = "";
        if inp.is_empty() {
            self.state+=1;
            self.first_str = true;
            if self.state > 1 {
                for a in 0..self.seeds.len() {
                    if self.mapped.get(&a) != None {
                        self.seeds[a] = *self.mapped.get(&a).unwrap();
                    };
                };
            }
            println!("> {:?}", self.seeds);
        } else {
            match self.state {
                0 => {
                    match parse(inp.as_str(), TAGS[self.state]) {
                        Ok((_s, r)) => self.seeds = r,
                        Err(_) => panic!("Failed to parse first line!"),
                    }
                },
                1..=7 => {
                    let st = if self.first_str { TAGS[self.state] } else {""};
                    match parse(inp.as_str(), st) {
                        Ok((_s, r)) => {
                            if !self.first_str {
                                if r.len() != 3 { panic!("Incorrect map lenghth!") };
                                for a in 0..self.seeds.len() {
                                    if self.seeds[a] >= r[1] {
                                        let d = self.seeds[a] - r[1];
                                        if d < r[2] {
                                            self.mapped.insert(a,r[0]+d);
                                            //println!("{} -> {}", self.seeds[a], r[0]+d);
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => panic!("Parsing error!"),
                    }
                   /* if self.first_str {println!(">>{}", TAGS[self.state])}; */
                },
                _ => (),
            }
            self.first_str = false;
        }
    }

    fn result(&mut self) -> String
    {
       let mut min = u32::MAX;
       for a in self.seeds.iter() {
           if *a < min { min = *a };
       }

       return min.to_string();
    }
}
