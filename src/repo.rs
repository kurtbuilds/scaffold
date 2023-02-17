use std::fs;
use std::path::PathBuf;
use crate::Template;
use anyhow::{Result, anyhow};
use expanduser::expanduser;
use ignore::{Walk};
use crate::template::File;

pub struct Options {
    pub template_dir: PathBuf,
    pub config_path: PathBuf,
    pub force: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            template_dir: expanduser("~/.scaffold").unwrap(),
            config_path: PathBuf::from("~/.scaffold/config.toml"),
            force: false,
        }
    }
}

/// Prepare the template directory if it doesn't exist already...
pub fn ensure_exists(options: &Options) -> Result<()> {
    if options.template_dir.exists() {
        return Ok(())
    }
    fs::create_dir_all(&options.template_dir)?;
    git_repository::init(&options.template_dir)?;
    Ok(())
}

pub fn save_template(template: &Template, options: &Options) -> Result<()> {
    ensure_exists(options).unwrap();

    let template_path = options.template_dir.join(&template.name);
    if template_path.exists() && !options.force {
        return Err(anyhow!("Template already exists. Use --force to overwrite."))
    }
    fs::create_dir_all(&template_path).unwrap();
    for file in template.files.iter() {
        let file_path = template_path.join(&file.relative_path);
        let parent = file_path.parent().unwrap();
        fs::create_dir_all(parent).unwrap();
        fs::write(file_path, &file.contents).unwrap();
    }
    Ok(())
}

pub fn read_template(template_name: &str, options: &Options) -> Result<Template> {
    let template_path = options.template_dir.join(template_name);
    if !template_path.exists() {
        return Err(anyhow!("Template does not exist."))
    }
    let mut template = Walk::new(&template_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|f| f.file_type().map(|t| t.is_file()).unwrap_or(false))
        .map(|f| {
            let relative_path = f.path().strip_prefix(&template_path).unwrap().to_path_buf();
            let contents = fs::read_to_string(&f.path())?;
            Ok::<_, anyhow::Error>(File {
                relative_path,
                contents,
            })
        }).collect::<Result<Vec<_>, _>>()
        .map(|files| Template {
            name: template_name.to_string(),
            files,
            commands: vec![],
        })?;

    if let Some(idx) = template.files.iter().position(|f| f.relative_path.to_str().unwrap() == "command.scaffold") {
        let command_file = template.files.remove(idx);
        template.commands = command_file.contents.lines().map(|s| s.to_string()).collect();
    }
    Ok(template)
}


pub fn list_templates(options: &Options) -> Result<Vec<String>> {
    let template_names = fs::read_dir(&options.template_dir)
        .unwrap()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().ok().map(|t| t.is_dir()).unwrap_or(false))
        .map(|e| e.file_name().into_string().unwrap())
        .filter(|s| ![".git"].contains(&s.as_str()))
        .collect::<Vec<_>>();
    Ok(template_names)
}