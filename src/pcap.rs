
use pcap::{Activated, Capture, Device, Error};
use dialoguer::{theme::ColorfulTheme, Select};

fn read_packets<T: Activated>(mut capture: Capture<T>) {
    while let Ok(packet) = capture.next_packet() {
        println!("received packet!: {:?} ", packet.header);
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
pub fn packet_listen(cap:Device) {
    println!("{:#?}",cap);
    read_packets(cap.open().unwrap());
}
