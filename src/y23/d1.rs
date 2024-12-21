#[path = "../solve.rs"] pub mod solve;

const SPELL:[&str; 9] = ["one", "two", "three", "four", "five",
                "six", "seven", "eight", "nine"];

pub struct Dx {
    sum: u64,
}

impl solve::Solve for Dx {
    fn new() -> Dx 
    {
        Dx { sum: 0 }
    }

    fn process(&mut self, inp: &String)
    {
        let mut left: u64 = 0;
        let mut right: u64 = 0;
        let mut left_pos: usize = 10000;
        let mut right_pos: usize =0;

        for pos in 0..inp.len() {
            let c = inp.chars().nth(pos).unwrap();
            if c.is_digit(10) {
                if left == 0{
                    left = c.to_digit(10).unwrap().into();
                    left_pos = pos;
                }
                right = c.to_digit(10).unwrap().into();
                right_pos = pos;
            }
        }

        for idx in 0..9 {
            let mut pos = inp.find(SPELL[idx]); if pos != None { if pos.unwrap() < left_pos { left = (idx + 1) as u64; left_pos = pos.unwrap(); }
            }
            pos = inp.rfind(SPELL[idx]);
            if pos != None {
                if pos.unwrap() > right_pos {
                    right = (idx + 1) as u64;
                    right_pos = pos.unwrap();
                }
            }
        }

        let res = left * 10 + right;
        self.sum += res;
    }

    fn result(&mut self) -> String
    {
       return self.sum.to_string();
    }
}
