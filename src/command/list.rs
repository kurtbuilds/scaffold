use anyhow::Result;
use clap::Parser;


#[derive(Debug, Parser)]
pub struct List {
    pub name: Option<String>,
}

impl List {
    pub fn run(self) -> Result<()> {
        let options = crate::repo::Options::default();
        if let Some(name) = self.name {
            let template = crate::repo::read_template(&name, &options)?;
            eprintln!("Template {}:", template.name);
            for file in template.files.iter() {
                eprintln!("{}", file.relative_path.display());
            }
        } else {
            let templates = crate::repo::list_templates(&options)?;
            for name in templates.iter() {
                eprintln!("{}\t{}", name, options.template_dir.join(&name).display());
            }
        }
        Ok(())
    }
}