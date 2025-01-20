use anyhow::Result;
use crb::agent::Runnable;
use tui_crb::TuiApp;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = TuiApp::new().run().await;
    // Unblocking stdin
    std::process::exit(0);
}
