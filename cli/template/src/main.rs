use crate::app::App;
use crate::cli::Commands;
use clap::{Parser, Subcommand};
use cli::Cli;
use color_eyre::Result;

mod action;
mod app;
mod cli;
mod components;
mod config;
mod errors;
mod logging;
mod tui;

use gnostr_lib::run;

#[tokio::main]
async fn tui() -> Result<()> {
    crate::errors::init()?;
    crate::logging::init()?;

    let args = Cli::parse();
    let mut app = App::new(args.tick_rate, args.frame_rate)?;
    app.run().await?;
    Ok(())
}

fn main() {
    //let matches = clap::App::new("gnostr: a git+nostr workflow utility")
    //    //.author("		a git+nostr workflow utility")
    //    .version("v0.0.1")
    //    .setting(AppSettings::ArgRequiredElseHelp)
    //    .subcommand(SubCommand::with_name("guess"))
    //    .subcommand(SubCommand::with_name("variables"))
    //    .subcommand(SubCommand::with_name("expressions"))
    //    .subcommand(SubCommand::with_name("loops"))
    //    .subcommand(SubCommand::with_name("fibonacci"))
    //    .subcommand(SubCommand::with_name("ownership"))
    //    .subcommand(SubCommand::with_name("slice"))
    //    .subcommand(SubCommand::with_name("rectangle"))
    //    .subcommand(SubCommand::with_name("option"))
    //    .subcommand(SubCommand::with_name("json"))
    //    .subcommand(SubCommand::with_name("collections"))
    //    .subcommand(SubCommand::with_name("traits"))
    //    .subcommand(SubCommand::with_name("lifetimes"))
    //    .subcommand(SubCommand::with_name("testing"))
    //    .subcommand(SubCommand::with_name("subdir"))
    //    .subcommand(SubCommand::with_name("pointers"))
    //    .subcommand(SubCommand::with_name("refcell"))
    //    .subcommand(SubCommand::with_name("tree"))
    //    .subcommand(SubCommand::with_name("threads"))
    //    .subcommand(SubCommand::with_name("thread2"))
    //    .subcommand(SubCommand::with_name("either"))
    //    .subcommand(SubCommand::with_name("notification"))
    //    .subcommand(SubCommand::with_name("rest"))
    //    .subcommand(SubCommand::with_name("lists"))
    //    .subcommand(SubCommand::with_name("install"))
    //    .get_matches();

    //match matches.subcommand_name() {
    //    Some(name) => run(name),
    //    //None => panic!("no subcommand"),
    //    None => tui().expect("REASON"),
    //}

    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Add { name }) => {
            println!("{:?}", name);
        }
        None => {
            println!("Default:None");
            let _ = tui();
        }
    }
}
