#[path = "solve.rs"] pub mod solve;

pub struct D1 {
    sum: u64,
}

impl solve::Solve for D1 {
    fn new() -> D1 
    {
        D1 { sum: 0 }
    }

    fn process(&mut self, inp: &String)
    {
        let mut left: u64 = 0;
        let mut right: u64 = 0;

        for c in inp.chars() {
            if c.is_digit(10) {
                if left == 0{
                    left = c.to_digit(10).unwrap().into();
                }
                right = c.to_digit(10).unwrap().into();
            }
        }
        let res = left * 10 + right;
        self.sum += res;

        println!("Res = {}", res);
    }

    fn result(&mut self) -> String
    {
       return self.sum.to_string();
    }
}
