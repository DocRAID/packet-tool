
mod pcap;
fn main() {
    pcap::packet_listen(pcap::device_select());
}