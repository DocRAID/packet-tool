use std::{error::Error, io};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, widgets::ListState, Terminal};
mod function;
mod tui_set;

pub struct ToolState {
    packet: Vec<String>,
    mode: tui_set::Mode,
    list_state: ListState,
    filter_option: tui_set::FilterOption,
    filter_query: String,
    filter_list: Vec<String>, //더 추가 해야함.
}
impl ToolState {
    pub fn new() -> ToolState {
        ToolState {
            packet: vec![],
            mode: tui_set::Mode::List,
            list_state: ListState::default(),
            filter_option: tui_set::FilterOption::All,
            filter_query: String::new(),
            filter_list: vec![],
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    function::check_root()?;
    //device selection
    let device = function::device_select();

    function::packet_listen(device); // 세팅 다끝나면 넣는거로

    //setup tui
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = tui_set::run_app(&mut terminal, &mut ToolState::new());
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    if let Err(e) = result {
        println!("{}", e.to_string());
    }

    return Ok(());
}
