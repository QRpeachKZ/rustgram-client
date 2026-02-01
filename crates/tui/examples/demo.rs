//! Demo application for rustgram-tui.
//!
//! Run this example to see the TUI in action with mock data.
//!
//! # Usage
//!
//! ```bash
//! cargo run -p rustgram-tui --bin demo
//! ```

use std::error::Error;

use rustgram_tui::RustgramTuiApp;

fn main() -> Result<(), Box<dyn Error>> {
    // Print welcome message
    println!("Starting Rustgram TUI Demo...");
    println!();
    println!("Keyboard shortcuts:");
    println!("  q / Esc - Quit");
    println!("  j / ↓     - Move down");
    println!("  k / ↑     - Move up");
    println!("  Enter     - Select dialog / Send message");
    println!("  i         - Switch to input mode");
    println!("  h         - Switch to dialog list");
    println!("  Alt+Enter - New line in input");
    println!();
    println!("Press any key to start...");
    let _ = crossterm::event::read();

    // Create and run the TUI application
    let mut app = RustgramTuiApp::new()?;

    // Customize the theme if desired
    // app.set_theme(Theme::dracula());
    // app.set_theme(Theme::nord());
    // app.set_theme(Theme::light());

    app.run()?;

    println!("Thanks for trying Rustgram TUI!");
    Ok(())
}
