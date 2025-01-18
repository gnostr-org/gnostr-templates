pub use app::App;
use crossterm::event::Event;
use std::sync::mpsc;
use ratatui::Terminal;
pub mod app;

//fn main() -> color_eyre::Result<()> {
//    color_eyre::install()?;
//    let terminal = ratatui::init();
//    let result = App::new().run(terminal);
//    ratatui::restore();
//    result
//}
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Create a channel for sending events to the main thread
    //let (tx, rx) = mpsc::channel();

    // Spawn a separate thread to handle input events
    std::thread::spawn(move || {
        // ... (Handle input events and send them to the main thread via tx)
    });

    let mut terminal = ratatui::init();
    //let terminal = Terminal::new().expect("Couldn't create terminal");

    let result = App::new().run(terminal);
    ratatui::restore();
    result

    //// Main rendering loop
    //loop {
    //    result.draw(|f| app.render(f)).unwrap();

    //    // Handle events
    //    match rx.recv().unwrap() {
    //        app::TuiEvent::Tick => {
    //            // Handle periodic events
    //        }
    //        app::TuiEvent::Input(key) => match key.as_str() {
    //            "q" => break,
    //            "j" | "Down" => app.down(),
    //            "k" | "Up" => app.up(),
    //            _ => {}
    //        },
    //    }
    //}

    //terminal.clear().unwrap();
}
