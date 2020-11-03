use std::{error::Error, io};
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::Terminal;

fn main() -> Result<(), Box<dyn Error>> {

    // stdout initialization
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    Ok(())
}
