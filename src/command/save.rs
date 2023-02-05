use anyhow::{Result};
use clap::{Parser};
use crate::template::Template;


#[derive(Parser, Debug)]
pub struct Save {
    template_name: String,
    paths: Vec<String>,

    #[clap(long, short)]
    force: bool,
}

impl Save {
    pub fn run(self) -> Result<()>{
        let options = crate::repo::Options {
            force: self.force,
            ..crate::repo::Options::default()
        };
        let files = self.paths.into_iter().map(|path| {
            let path = std::path::Path::new(&path);
            let contents = std::fs::read_to_string(path)?;
            Ok(crate::template::File {
                relative_path: path.to_path_buf(),
                contents,
            })
        }).collect::<Result<Vec<_>>>()?;

        let template = Template {
            name: self.template_name,
            files,
            commands: vec![],
        };
        crate::repo::save_template(&template, &options)?;
        eprintln!("Template {} saved successfully. Contents:", &template.name);
        for file in template.files.iter() {
            eprintln!("{}", file.relative_path.display());
        }
        Ok(())
    }
}