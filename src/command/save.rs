use clap::{Parser};


#[derive(Parser, Debug)]
struct Save {
    template_name: String,
    paths: Vec<String>,
}

impl Save {
    pub fn run(self) {
        println!("Save: {:?}", self);
    }
}