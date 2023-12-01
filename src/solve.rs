pub trait Solve {
    fn new() -> Self;
    fn process(&mut self, inp: &String);
    fn result(&mut self) -> String;
}
