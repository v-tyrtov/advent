use nom::character::complete::{char, digit1, one_of};
use nom::combinator::map_res;
use nom::sequence::separated_pair;
use nom::multi::many1;
use nom::{Err, IResult};
use phf::phf_map;
use std::collections::BTreeSet;
use std::cmp::Ordering;
use std::fmt;

#[path = "solve.rs"] pub mod solve;


static CARD_VAL: phf::Map<char, u32> = phf_map!{
    '2' => 1,
    '3' => 2,
    '4' => 3,
    '5' => 4,
    '6' => 5,
    '7' => 6,
    '8' => 7,
    '9' => 8,
    'T' => 9,
    'J' => 10,
    'Q' => 11,
    'K' => 12,
    'A' => 13,
};

#[derive(Clone, Eq, PartialEq)]
struct Card {
    c: char
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        if CARD_VAL.get(&self.c) > CARD_VAL.get(&other.c) { return Ordering::Greater }
        if CARD_VAL.get(&self.c) < CARD_VAL.get(&other.c) { return Ordering::Less }
        Ordering::Equal
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Type {
    One = 1,
    Pair = 2,
    TwoPair = 3,
    Three = 4,
    House = 5,
    Four = 6,
    Five = 7
}

struct Game {
    cards: Vec<Card>,
    bid: u32,
    hand: Type,
}
impl Game {
    fn new(v: Vec<Card>, b: u32) -> Self {
        if v.len() != 5 { panic!("Wrong number of cards!") };

        let mut vs = v.clone();
        vs.sort();

        let mut it = vs.iter();
        let mut char = it.next().unwrap();

        let mut num = 1;
        let mut t = Type::One;

        for c in it {
            if c == char {
                num += 1;
            } else {
                t = match num {
                    1 => t,
                    2 => match t {
                        Type::One => Type::Pair,
                        Type::Pair => Type::TwoPair,
                        Type::Three => Type::House,
                        _ => panic!("Err 0")
                    }
                    3 => match t {
                        Type::One => Type::Three,
                        Type::Pair => Type::House,
                        _ => panic!("Err 1")
                    }
                    4 => Type::Four,
                    5 => Type::Five,
                    _ => panic!("Err 2")
                };
                num = 1;
                char= c;
            }

        }
            t = match num {
                    1 => t,
                    2 => match t {
                        Type::One => Type::Pair,
                        Type::Pair => Type::TwoPair,
                        Type::Three => Type::House,
                        _ => panic!("Err 0")
                    }
                    3 => match t {
                        Type::One => Type::Three,
                        Type::Pair => Type::House,
                        _ => panic!("Err 1")
                    }
                    4 => Type::Four,
                    5 => Type::Five,
                    _ => panic!("Err 2")
                };
        
        Game{cards: v, bid: b, hand: t}
    }
}
    

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {

        let mut ret = Ordering::Equal;

        if (self.hand as u8) < (other.hand as u8) { ret = Ordering::Less; }
        if (self.hand as u8) > (other.hand as u8) { ret = Ordering::Greater; }

        if ret == Ordering::Equal {

        for n in 0..self.cards.len() {
            if self.cards[n] < other.cards[n] { ret = Ordering::Less; break }
            if self.cards[n] > other.cards[n] { ret = Ordering::Greater; break }
        }
        }
        //println!("{:?} {:?} {:?}", self, ret, other);
        ret
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Game {}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cards.iter().for_each(|g| { write!(f, "{}", g.c).ok(); });
        write!(f, " {:?} ", self.hand).ok();
        write!(f, " - {}", self.bid)
    }
}

pub struct D7 {
    games: BTreeSet<Game>,
}

impl D7 {

}

fn p_card(inp: &str) -> IResult<&str, Card> {
    let (s, c) = one_of("23456789TJQKA")(inp)?;
    let card = Card{c};
    Ok((&s, card))
}

fn parse<'a>(inp: &'a str) -> IResult<&'a str, Game> {
    //let mut res = Game{cards: vec![], bid: 0};

    let uint = map_res(digit1, |s: &str| s.parse::<u32>());
    let (o_str, (cards, bid)) = separated_pair(many1(p_card), char(' '), uint)(inp)?;

    let res = Game::new(cards, bid);

    return Ok((o_str, res));
}

impl solve::Solve for D7 {
    fn new() -> D7
    {
        D7 {
            games: BTreeSet::new()
        }
    }

    fn process(&mut self, inp: &String)
    {
        let mut _s = "";
        if !inp.is_empty() {
            match parse(inp.as_str()) {
                Ok((_s, r)) => self.games.insert(r),
                Err(Err::Error(s)) => panic!("Parsing Error for {}", s),
                Err(_) => panic!("Unexpected parsing err!"),
            };
        }
    }

    fn result(&mut self) -> String
    {

        let mut res = 0;
        let mut cnt = 1;

        for a in &self.games {
            res += a.bid * cnt;
            cnt += 1;
        }

        return res.to_string();
    }
}
