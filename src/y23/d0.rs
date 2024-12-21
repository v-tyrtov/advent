//pub trait Solve {
//    fn new() -> Self;
//    fn process(&mut self, inp: &String);
//    fn result(&mut self) -> String;
//}
//

#[path = "../solve.rs"] pub mod solve;

pub struct Dx {
    curr: u64,
    max1: u64,
    max2: u64,
    max3: u64,
}

impl Dx {
    fn update_max(&mut self)
    {
        if self.max1 < self.curr {
            self.max3 = self.max2;
            self.max2 = self.max1;
            self.max1 = self.curr;
            self.curr = 0;
        }
        if self.max2 < self.curr {
            self.max3 = self.max2;
            self.max2 = self.curr;
            self.curr = 0;
        }
        if self.max3 < self.curr {
            self.max3 = self.curr;
        }

        self.curr = 0;
    }
}

impl solve::Solve for Dx {
    fn new() -> Dx
    {
        Dx { curr: 0, max1: 0, max2: 0, max3: 0 }
    }

    fn process(&mut self, inp: &String)
    {
        if inp.is_empty() {
            self.update_max();
        } else {
            let cal: u64 = inp.parse().unwrap();
            self.curr += cal;
        }
    }

    fn result(&mut self) -> String
    {
       self.update_max(); 
       let max = self.max1 + self.max2 + self.max3;
       return max.to_string();
    }
}
