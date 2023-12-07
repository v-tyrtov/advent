use std::vec::Vec;
#[path = "solve.rs"] pub mod solve;


pub struct D3 {
    map: Vec<Vec<char>>
}

impl D3 {
    fn is_around(&self, v: usize, h: usize) -> bool {
        let mut res = false;
        if (v >0) && (self.map[v-1][h] != '.') { res = true }
        if (v < (self.map.len()-1)) && (self.map[v+1][h] != '.') { res = true }
        return res;
    }

}

impl solve::Solve for D3 {
    fn new() -> D3 
    {
        D3 { map: Vec::new() }
    }

    fn process(&mut self, inp: &String)
    {
        self.map.push(inp.chars().collect());
    }

    fn result(&mut self) -> String
    {
        let mut sum = 0;
        for v_idx in 0..self.map.len() {
            let mut sym_around = false;
            let mut num = 0;
            let mut is_num = false;
            let mut str = String::from("");
            for h_idx in 0..self.map[0].len() {
                if self.map[v_idx][h_idx].is_digit(10) {
                    is_num = true;
                    num = num * 10 + self.map[v_idx][h_idx].to_digit(10).unwrap();
                    sym_around = sym_around || self.is_around(v_idx, h_idx); 

                } else {
                    if is_num == true {
                        sym_around = sym_around || self.is_around(v_idx, h_idx) || self.map[v_idx][h_idx] != '.';
                        if sym_around == true {
                            sum += num; 
                            str.push_str(&num.to_string());
                        }
                    }
                    is_num = false;
                    num = 0;
                    sym_around = self.is_around(v_idx, h_idx) || self.map[v_idx][h_idx] != '.';
                    str.push('.');
                }
            }
            if is_num && sym_around {
                sum += num; 
                str.push_str(&num.to_string());
            }
//            println!("{}", str);
        }
        return sum.to_string();
    }
}
