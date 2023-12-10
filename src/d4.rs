use std::vec::Vec;
#[path = "solve.rs"] pub mod solve;


pub struct D4 {
    sum: u32,
}

fn parse(inp: &String) -> Vec<u32> {
    
    let mut ints = Vec::new();
    
    for a in inp.split(' ') {
        match a.parse::<u32>() {
            Ok(n) => ints.push(n),
            Err(_) => continue
        }
    }
        
    return ints;
}

impl solve::Solve for D4 {
    fn new() -> D4 
    {
        D4 { sum: 0 }
    }

    fn process(&mut self, inp: &String)
    {
        let mut chunk = inp.split(':');
        chunk.next();

        chunk = chunk.next().unwrap().split('|');

        let wins = parse(&String::from(chunk.next().unwrap()));
        let numb = parse(&String::from(chunk.next().unwrap()));
        
        let mut str = String::new();
        let mut inc = 1;
        let mut res = 0;
        for a in numb.iter() {
            if wins.contains(a) {
                str.push_str(&a.to_string());
                str.push(' ');
                res = inc;
                inc *= 2;
            }
        }
        //println!("> {} = {}", str, res);
        self.sum += res;

    }

    fn result(&mut self) -> String
    {
       return self.sum.to_string();
    }
}
