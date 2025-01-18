use std::io;

use ratatui::{backend::CrosstermBackend, Terminal};
use std::process::Command;
use tokio::process::Command as TokioCommand;
use tokio::task;

use crate::{
    app::{App, AppResult},
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};

pub mod app;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;

#[tokio::main]
async fn main() -> AppResult<()> {
//preprocesses
    // Run a non-async command
    let non_async_handle = task::spawn(async {
        let output = Command::new("ls")
            .arg("-l")
            .output()
            .expect("Failed to execute non-async command");
        println!("Non-async output: {:?}", output);
    });

    // Run an async command
    let async_handle = task::spawn(async {
        let output = TokioCommand::new("ping")
            .arg("127.0.0.1")
            .arg("-c")
            .arg("1")
            .output()
            .await
            .expect("Failed to execute async command");
        println!("Async output: {:?}", output);
    });
    // Run an async command
    let gnostr_async_handle = task::spawn(async {
        let output = TokioCommand::new("gnostr-sha256")
            .output()
            .await
            .expect("Failed to execute async command");
        println!("Async output: {:?}", output);
    });

    // Wait for both tasks to complete
    non_async_handle.await?;
    async_handle.await?;
    gnostr_async_handle.await?;




    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
