use std::vec;
#[path = "../solve.rs"] pub mod solve;

pub struct Dx {
    puzzle: Vec<Vec<char>>,
}

struct IterDx<'a> {
    data: &'a Dx,
    x: usize,
    y: usize,
}

type IterDxVertH<'a> = IterDx<'a>;

impl<'a> Iterator for IterDxVertH<'a> {
    type Item = &'a char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.data.puzzle[0].len() {
            self.x += 1;
            Some(&self.data.puzzle[self.y - 1][self.x - 1])
        } else {
            None
        }
    }
}

struct IterDxVert<'a> {
    data: &'a Dx,
    x: usize,
    y: usize,
}

impl<'a> Iterator for IterDxVert<'a> {
    type Item = IterDxVertH<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.data.puzzle.len() {
            self.y += 1;
            Some(self.data.iter_h(self.y))
        } else {
            None
        }
    }
}

impl Dx {
    fn iter_h(&self, pos: usize) -> IterDxVertH {
        IterDxVertH{data: &self, x: 0, y: pos}
    }
    fn iter_v(&self) -> IterDxVert {
        IterDxVert{data: &self, x: 0, y: 0}
    }
}

//struct IterDxHoriz {
//    data: &Dx,
//    pos: usize,
//}

const WORD:[char; 4] = ['X', 'M', 'A', 'S'];
const DROW:[char; 4] = ['S', 'A', 'M', 'X'];

//#[derive(Debug)]
struct Find<'a> {
    pos: usize,
    idx: usize,
    it: IterDx<'a>,
}

fn check1<T: Iterator> (mut it: T, mut look: Find, word: [char; 4]) -> Find  where <T as Iterator>::Item: PartialEq<char> {
   let the_ch = it.next().unwrap();
            if the_ch != word[look.idx] {
                look.pos = usize::MAX;
                look.idx = 0;
            }
            if the_ch == word[look.idx] {
                if look.it == None {
                    look.it = it;
                };
                if look.idx == 3 {
                    // the word is found
                    //println!(">> {:?}", forward);

                    let mut i = 0;
                    for ch in word {
                        *look.it.next().unwrap() = ch;
                    }
                    look.it = None;
                    look.idx = 0;

                } else {
                    // move to the next char
                    look.idx += 1;
                };
            }
 
        look
    }


fn find_word(inp: &Vec<Vec<char>>) -> Vec<Vec<char>>
{
    let mut out = vec![vec!['.'; inp[0].len()]; inp.len()];

    let mut forward = Find {pos: usize::MAX, idx: 0};
    let mut backward = Find {pos: usize::MAX, idx: 0};

    let mut check = |a: usize, b: usize, mut look: Find, word: [char; 4]| -> Find {
            if inp[a][b] != word[look.idx] {
                look.pos = usize::MAX;
                look.idx = 0;
            }
            if inp[a][b] == word[look.idx] {
                if look.pos == usize::MAX {
                    look.pos = b;
                };
                if look.idx == 3 {
                    // the word is found
                    //println!(">> {:?}", forward);
                    let mut i = 0;
                    for ch in word {
                        out[a][look.pos+i] = ch;
                        i += 1;
                    }
                    look.pos = usize::MAX;
                    look.idx = 0;

                } else {
                    // move to the next char
                    look.idx += 1;
                };
            }
 
        look
    };


    for a in 0..inp[0].len() {
        for b in 0..inp.len() {
            forward = check(a, b, forward, WORD);
            backward = check(a, b, backward, DROW);
        }
    }

    /*
    forward = Find {pos: usize::MAX, idx: 0};
    backward = Find {pos: usize::MAX, idx: 0};

    for a in 0..inp.len() {
        for b in 0..inp[0].len() {
            forward = check(a, b, forward, WORD);
            backward = check(a, b, backward, DROW);
        }
    }
    */

    out
}

impl solve::Solve for Dx {
    fn new() -> Dx
    {
        Dx { puzzle: vec![] }
    }

    fn process(&mut self, inp: &String)
    {
        self.puzzle.push(inp.chars().collect());
    }

    fn result(&mut self) -> String
    {
        println!("Result >");
        let out = find_word(&self.puzzle);
        for a in &out {
            println!("{}", String::from_iter(a));
        }
        return 0.to_string();
    }
}
