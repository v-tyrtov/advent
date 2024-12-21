use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::bytes::complete::tag;
use nom::IResult;
use std::collections::BTreeSet;

#[path = "../solve.rs"] pub mod solve;


const TAGS:[&str; 8] =
            ["seeds: ",
             "seed-to-soil map:",
             "soil-to-fertilizer map:",
             "fertilizer-to-water map:",
             "water-to-light map:",
             "light-to-temperature map:",
             "temperature-to-humidity map:",
             "humidity-to-location map:"];

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Range {
    start: u64,
    len: u64
}

pub struct Dx {
    state: usize,
    first_str: bool,
    seeds: BTreeSet<Range>,
    mapped: BTreeSet<Range>,
}

impl Dx {

}

fn parse<'a>(inp: &'a str, pattern: &str) -> IResult<&'a str, Vec<u64>> {
    let mut res = vec![];
    let mut o_str: &str = "";
    let parse = map_res(digit1, |s: &str| s.parse::<u64>());
    let (seed_str, _t) = tag(pattern)(inp)?;
    if seed_str.len() > 0 {
        (o_str, res) = separated_list0(tag(" "), parse)(seed_str)?;
    };

    return Ok((o_str, res));
}

fn contains(val: &Range, check: &Vec<u64>) ->
        Result<(BTreeSet<Range>, BTreeSet<Range>), u32> {
    let mut res: BTreeSet<Range> = BTreeSet::new();
    let mut res_n: BTreeSet<Range> = BTreeSet::new();

    if val.start < check[1] {
        if (val.start + val.len) <= check[1] { return Err(0) }
        else {
            res.insert(Range{start: val.start, len: check[1] - val.start});
            let end_r = val.start + val.len;
            if end_r <= (check[1] + check[2]) {
                res_n.insert(Range{start: check[0], len: end_r - check[1]});
            } else {
                res_n.insert(Range{start: check[0], len: check[2]});
                res.insert(Range{start: check[1] + check[2],
                                    len: end_r - check[1] - check[2]});
            }
        }
    } else {
        if val.start < (check[1] + check [2]) {
            let end_r = val.start + val.len;
            if end_r <= (check[1] + check[2]) {
                res_n.insert(Range{start: check[0] + val.start - check[1],
                                        len: val.len});
            } else {
                res_n.insert(Range{start: check[0] + val.start - check[1],
                                    len: check[2] - (val.start - check[1])});
                res.insert(Range{start: check[1] + check[2],
                                    len: end_r - check[1] - check[2]});
            }
        } else { return Err(0) }

    }
    return Ok((res, res_n));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_contains() {
        let b = Range{start: 11, len: 10};
        assert_eq!(contains(&b, &vec![100, 30, 10]),Err(0));
        assert_eq!(contains(&b, &vec![100, 0, 10]),Err(0));

        let mut old = BTreeSet::new();
        old.insert(Range{start: 11, len: 4});
        let mut new = BTreeSet::new();
        new.insert(Range{start: 100, len: 6});
        assert_eq!(contains(&b, &vec![100, 15, 10]),Ok((old,new)));

        old = BTreeSet::new();
        old.insert(Range{start: 17, len: 4});
        new = BTreeSet::new();
        new.insert(Range{start: 100, len: 6});

        assert_eq!(contains(&b, &vec![99, 10 , 7]),Ok((old, new)));
    }
}

impl solve::Solve for Dx {
    fn new() -> Dx
    {
        Dx {
            state: 0,
            first_str: false,
            seeds: BTreeSet::new(),
            mapped: BTreeSet::new()
        }
    }

    fn process(&mut self, inp: &String)
    {
        let mut _s = "";
        if inp.is_empty() {
            self.state+=1;
            self.first_str = true;
            if self.state > 1 {
                self.seeds.append(&mut self.mapped);
                self.mapped = BTreeSet::new();
            }
            println!("> {:?}", self.seeds);
        } else {
            match self.state {
                0 => {
                    match parse(inp.as_str(), TAGS[self.state]) {
                        Ok((_s, r)) => for a in 0..r.len()/2 {
                                            self.seeds.insert(Range{start: r[a*2], len: r[a*2+1]});
                                        }
                        Err(_) => panic!("Failed to parse first line!"),
                    }
                },
                1..=7 => {
                    let st = if self.first_str { TAGS[self.state] } else {""};
                    match parse(inp.as_str(), st) {
                        Ok((_s, r)) => {
                            if !self.first_str {
                                if r.len() != 3 { panic!("Incorrect map lenghth!") };
                                let mut app_v = BTreeSet::new();
                                self.seeds.retain(|s| {
                                    match contains(s, &r) {
                                        Ok((mut old_v, mut new_v)) => {
                                            self.mapped.append(&mut new_v);
                                            app_v.append(&mut old_v);
                                            false
                                        },
                                        Err(_) => true,
                                    }
                                });
                                //println!("<<< {:?}", app_v);
                                self.seeds.append(&mut app_v);
                            }
                        }
                        Err(_) => panic!("Parsing error!"),
                    }
                },
                _ => { println!("End") },
            }
            self.first_str = false;
        }
    }

    fn result(&mut self) -> String
    {

        let min = self.seeds.first().unwrap();

        return min.start.to_string();
    }
}
