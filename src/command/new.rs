use clap::{Parser};


#[derive(Parser, Debug)]
struct New {
    template: String,
    values: Vec<String>,
}

impl New {
    pub fn run(self) {
        println!("New: {:?}", self);
    }
}