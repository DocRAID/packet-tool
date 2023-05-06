use std::{error::Error, io, time::Duration};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen}, execute, event::EnableMouseCapture};
use tui::{backend::CrosstermBackend, Terminal};
pub enum Mode {
    List,
    Detail,
    Filter,
}
enum FilterOption {
    Source,
    Destination,
    Protocol,
}

pub fn setup_tui() -> Result<(), Box<dyn Error>>{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // let result = run_app(&mut terminal, &mut state);
    Ok(())
}