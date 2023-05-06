use crossterm::event;
use crossterm::event::Event::Key;

use tui::Frame;
use tui::{
    backend::{Backend},
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
    loop {
        terminal.draw(|f| ui(f, state))?;

        if let Key(key) = event::read()? {
            match state.mode {
                //기능정의
                Mode::List => todo!(),
                Mode::Detail => todo!(),
                Mode::Filter => todo!(),
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, state: &mut ToolState) {}
