use anyhow::Result;
mod command;
mod repo;
mod template;

use clap::{Parser, Subcommand, CommandFactory};
use command::*;
use template::Template;


#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[clap(short, long)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Add a scaffold to the current directory
    New(New),
    /// Save a scaffold template to the local cache
    Save(Save),
    /// List all the templates available
    List(List),
    /// Generate shell completions
    ShellCompletions {
        shell: String,
    }
}

#[derive(Parser, Debug)]
struct Create {
    foo: String,
}


fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::ShellCompletions { .. } => {
            clap_complete::generate(clap_complete::shells::Bash, &mut Cli::command(), env!("CARGO_BIN_NAME"), &mut std::io::stdout());
            eprintln!("INSTRUCTIONS:");
            eprintln!("mkdir -p ~/.bash_completion/");
            eprintln!("scaffold shell-completions bash > ~/.bash_completion/scaffold");
            eprintln!("echo 'source ~/.bash_completion/*' >> ~/.bash_profile");
            Ok(())
        }
        Command::New(new) => new.run(),
        Command::Save(save) => save.run(),
        Command::List(list) => list.run(),
    }
}
