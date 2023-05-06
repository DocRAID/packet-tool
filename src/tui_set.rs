use std::{error::Error, io};

use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::ToolState;
pub enum Mode {
    List,
    Detail,
    Filter,
}
pub enum FilterOption {
    All,
    Source,
    Destination,
    Protocol,
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    state: &mut ToolState,
) -> Result<(), std::io::Error> {
    Ok(())
}
