use std::vec::Vec;
#[path = "solve.rs"] pub mod solve;


const COLORS:[&str; 3] = ["red", "green", "blue"];

struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

pub struct D2 {
    sum: u32,
}

fn parse(inp: &String) -> (u32, Vec<Game>) {
    let mut chunk = inp.split(':');
    let game_n = chunk.next().unwrap().strip_prefix("Game ").unwrap().to_string().parse::<u32>().unwrap(); 

    let mut games: Vec<Game> = Vec::new();

    for round_s in chunk.next().unwrap().split(';') {
        
        let mut game = Game {red: 0, green: 0, blue: 0};
        for game_s in round_s.split(',') {
            let mut g_chunk = game_s.trim().split(" ");
            let val_s = g_chunk.next().unwrap();

            let val = val_s.parse::<u32>().unwrap();
            let ch = g_chunk.next().unwrap();
            if ch.find("red") != None {
                game.red = val;
            }
            if ch.find("green") != None {
                game.green = val;
            }
            if ch.find("blue") != None {
                game.blue = val;
            }
        }
        games.push(game);
    }
    return (game_n, games);
}

impl solve::Solve for D2 {
    fn new() -> D2 
    {
        D2 { sum: 0 }
    }

    fn process(&mut self, inp: &String)
    {
        let (n, games) = parse(inp);

        let mut max_g: Game = Game {red:0, green: 0, blue: 0};

        for g in games {
            if max_g.red < g.red {
                max_g.red = g.red;
            }
            if max_g.green < g.green {
                max_g.green = g.green;
            }
            if max_g.blue < g.blue {
                max_g.blue = g.blue;
            }
        }
        self.sum += max_g.red * max_g.green * max_g.blue;
    }

    fn result(&mut self) -> String
    {
       return self.sum.to_string();
    }
}
