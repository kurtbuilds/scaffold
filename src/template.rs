use std::path::PathBuf;

#[derive(Debug)]
pub struct File {
    pub relative_path: PathBuf,
    pub contents: String,
}

#[derive(Debug)]
pub struct Template {
    pub name: String,
    pub files: Vec<File>,
    pub commands: Vec<String>,
}