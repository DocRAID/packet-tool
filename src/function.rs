use pcap::{Activated, Capture, Device, };
use dialoguer::{theme::ColorfulTheme, Select};
use is_root::is_root;
use console::style;

pub fn run() -> Result<(),std::fmt::Error>{
    if !is_root(){
        eprintln!("{}",style("Root permissions are required to access socket information.\nretry as root user").red().bold());
        return Err(std::fmt::Error);
    }
    packet_listen(device_select());
    return Ok(());
}

fn read_packets<T: Activated>(mut capture: Capture<T>) {
    while let Ok(packet) = capture.next_packet() {
        println!("received packet!: {:?} ", packet);
    }
}

pub fn device_select() -> Device{
    let devices: Vec<Device> = Device::list().unwrap();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your network device")
        .default(0)
        .items(&device_list(devices.clone()))
        .interact()
        .unwrap();
    match devices.get(selection) {
        Some(device) => return device.clone(),
        None => panic!("Something was worring"),
    }
}
fn device_list(devices:Vec<Device>) -> Vec<String>{
    let mut device_name:Vec<String> = Vec::new();
    for device in devices {
        device_name.push(device.name);
    }
    device_name
}
pub fn packet_listen(device:Device) {
    
    let mut cap = pcap::Capture::from_device(device)
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();
    
    read_packets(cap);
}
