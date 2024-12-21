use std::vec::Vec;
#[path = "../solve.rs"] pub mod solve;


pub struct Dx {
    len: usize,
    copies: Vec<u32>,
    result: Vec<u32>,
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

impl solve::Solve for Dx {
    fn new() -> Dx 
    {
        Dx { len: 0, copies: vec![0; 256], result: vec![0; 256] }
    }

    fn process(&mut self, inp: &String)
    {
        let mut chunk = inp.split(':');
        let card_num = chunk.next().unwrap().trim_start_matches("Card").trim()
                            .parse::<u32>().unwrap() as usize;


        chunk = chunk.next().unwrap().split('|');

        let wins = parse(&String::from(chunk.next().unwrap()));
        let numb = parse(&String::from(chunk.next().unwrap()));
        
        let mut str = String::new();
        let mut res = 0;
        for a in numb.iter() {
            if wins.contains(a) {
                str.push_str(&a.to_string());
                str.push(' ');
                res += 1;
            }
        }

        self.result[self.len] += 1 + self.copies[self.len];
        if res > 0 {
            for i in self.len+1.. self.len+1+res {
                self.copies[i] += self.result[self.len];
            }
        }
        //println!("> {} = {}", str, res);
        self.len += 1;
        assert!( card_num == self.len, "Unexpected card num {}", card_num);

    }

    fn result(&mut self) -> String
    {
        let mut str = String::new();      
        let mut str1 = String::new();      
        let mut sum = 0;
        for i in 0..self.len {
            str.push_str(&self.result[i].to_string());
            str1.push_str(&self.copies[i].to_string());
            str.push(' ');
            str1.push(' ');
            sum += self.result[i];
        }

        //println!("> {} | {}", str, str1);

        return sum.to_string();
    }
}
