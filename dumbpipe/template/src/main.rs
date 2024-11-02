mod lib;
mod app;
use crate::app::App;
use {{project-name | snake_case}}::*;
use clap::{Parser, Subcommand};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();


    color_eyre::install().expect("REASON");
    let terminal = ratatui::init();

    //let app_result = {{project-name | snake_case}}::app::App::new().run(terminal);
    let app_result = App::new().run(terminal);
    //let app_result = iroh_net::endpoint::Source().run(terminal);
    ratatui::restore();
    //app_result

    //let args = Args::parse();
    let args = Args::parse();
    let res = match args.command {
        Commands::Listen(args) => listen_stdio(args).await,
        Commands::ListenTcp(args) => listen_tcp(args).await,
        Commands::Connect(args) => connect_stdio(args).await,
        Commands::ConnectTcp(args) => connect_tcp(args).await,
    };
    match res {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1)
        }
    }
}
