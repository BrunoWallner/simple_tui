pub trait Application {
    fn new() -> Self;
    fn update(&mut self, msg: Vec<String>);
    fn view(&mut self);
    fn run(&mut self);
}