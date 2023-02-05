use std::fs;
use anyhow::{anyhow, Result};
use clap::{Parser};


#[derive(Parser, Debug)]
pub struct New {
    template: String,
    values: Vec<String>,

    #[clap(long, short)]
    force: bool,
}

impl New {
    pub fn run(self) -> Result<()> {
        let options = crate::repo::Options::default();
        let template = crate::repo::read_template(&self.template, &options)?;
        if !self.force {
            let mut need_force = false;
            for file in template.files.iter() {
                if file.relative_path.exists() && !self.force {
                    need_force = true;
                    eprintln!("File {} already exists.", file.relative_path.display());
                }
            }
            if need_force {
                return Err(anyhow!("Use --force to overwrite existing files."));
            }
        }

        for file in template.files.iter() {
            let path = &file.relative_path;
            let parent = path.parent().unwrap();
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
            fs::write(path, &file.contents)?;
            eprintln!("{}: Wrote file.", path.display());
        }
        Ok(())
    }
}