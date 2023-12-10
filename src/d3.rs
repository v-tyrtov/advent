use std::vec::Vec;
use std::collections::HashMap;
#[path = "solve.rs"] pub mod solve;


struct GearTst {
    is_gear: bool,
    v: usize,
    h: usize,
}

pub struct D3 {
    map: Vec<Vec<char>>,
    tst: GearTst,
    gears: HashMap<(usize, usize), Vec<u32>>,
}


impl D3 {
    fn is_around(&self, v: usize, h: usize) -> bool {
        let mut res = false;
        if (v >0) && (self.map[v-1][h] != '.') { res = true }
        if (v < (self.map.len()-1)) && (self.map[v+1][h] != '.') { res = true }
        return res;
    }

    fn tst_gear(&mut self, v: usize, h: usize) {
        if self.map[v][h] == '*' {
            assert!(!self.tst.is_gear, "Second gear is nearby {}, {}!", self.tst.v,self.tst.h);
            self.tst.is_gear = true;
            self.tst.v = v;
            self.tst.h = h;
        }
    }

    fn check_gear_around(&mut self, v: usize, h: usize) {
        if v > 0 { self.tst_gear(v-1,h) };
        if v < (self.map.len()-1) { self.tst_gear(v+1, h) };
    }

    fn check_gear(&mut self, v: usize, h: usize) {
        self.tst_gear(v, h);
        self.check_gear_around(v, h);
    }

    fn is_gear(&self) -> bool {
        return self.tst.is_gear;
    }

    fn reset_gear(&mut self) {
        self.tst = GearTst {is_gear: false, v: 0, h: 0 };
    }

    fn add_gear(&mut self, n: u32) {
        assert!(self.tst.is_gear, "There is no gear to push!");
        self.gears.entry((self.tst.v, self.tst.h))
            .and_modify(|v| { v.push(n) })
            .or_insert(vec![n]);
    }
}

impl solve::Solve for D3 {
    fn new() -> D3 
    {
        D3 { map: Vec::new(), gears: HashMap::new(),
              tst: GearTst{is_gear: false, v: 0, h: 0} }
    }

    fn process(&mut self, inp: &String)
    {
        self.map.push(inp.chars().collect());
    }

    fn result(&mut self) -> String
    {
        let mut sum = 0;
        for v_idx in 0..self.map.len() {
            let mut num = 0;
            let mut is_num = false;
            let mut str = String::from("");
            for h_idx in 0..self.map[0].len() {
                if self.map[v_idx][h_idx].is_digit(10) {
                    is_num = true;
                    num = num * 10 + self.map[v_idx][h_idx].to_digit(10).unwrap();
                    self.check_gear_around(v_idx,h_idx);
                } else {
                    if is_num {
                        self.check_gear(v_idx,h_idx);
                        if self.is_gear() {
                            sum += num; 
                            str.push_str(&num.to_string());
                            self.add_gear(num);
                        }
                    }
                    is_num = false;
                    num = 0;
                    self.reset_gear();
                    self.check_gear(v_idx, h_idx);
                    str.push('.');
                }
            }
            if is_num && self.is_gear(){
                sum += num; 
                str.push_str(&num.to_string());
                self.add_gear(num);
            }
//            println!("{}", str);
        }

        sum = 0;
        for (_k, gg) in self.gears.iter() {
            if gg.len() > 2 {
                println!("Brocken gear!");
            }
            if gg.len() == 2 {
                sum += gg[0]*gg[1];
            }
        }

        return sum.to_string();
    }
}
