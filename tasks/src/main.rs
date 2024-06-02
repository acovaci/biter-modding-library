use clap::Parser;

mod _build;
mod _deploy;
mod _restore;
mod _run;
mod _setup;

mod fs;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Build,
    Deploy,
    Restore,
    Run,
    Setup,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Build => _build::build().expect("Failed to build library."),
        Command::Deploy => _deploy::deploy().expect("Failed to deploy library."),
        Command::Restore => _restore::restore(),
        Command::Run => _run::run(),
        Command::Setup => _setup::setup(),
    }
}
