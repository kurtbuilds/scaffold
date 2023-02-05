mod command;
mod repo;

use clap::{Parser, Subcommand, ValueEnum, Args};
use command::*;


#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    command: Command,

    #[clap(short, long)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
    New(New),
    Save(Save),
    ShellCompletions,
}

#[derive(Parser, Debug)]
struct Create {
    foo: String,
}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::ShellCompletions => {
            clap_complete::generate(clap_complete::shells::Bash, &mut Cli, "myapp", &mut std::io::stdout());
        }
        Command::New(new) => {
            new.run();
        }
        Command::Save(save) => {
            save.run();
        }
    }
}
