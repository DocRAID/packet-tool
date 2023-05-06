use std::error::Error;
mod function;
mod tui_set;

struct ToolState {
    packet: Vec<String>,
    mode: tui_set::Mode,
    filter_list:Vec<String>
}
fn main() -> Result<(), Box<dyn Error>> {
    function::check_root()?;
    //device selection
    let device = function::device_select();

    //setup tui
    // tui_set::setup_tui()?;

    function::packet_listen(device);
    
    return Ok(());
}
