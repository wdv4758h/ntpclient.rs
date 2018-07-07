extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;


use std::net::UdpSocket;
use packed_struct::prelude::*;


#[derive(Debug, Default)]
#[derive(PackedStruct)]
#[packed_struct(bit_numbering="msb0")]
#[packed_struct(endian="msb")]
pub struct NTPPacket {
    #[packed_field(bits="0..=1")]
    leap_indicator: u8,
    #[packed_field(bits="2..=4")]
    version_number: u8,
    #[packed_field(bits="5..=7")]
    mode: u8,
    #[packed_field(bits="8..=15")]
    stratum: u8,
    #[packed_field(bits="16..=23")]
    poll: u8,
    #[packed_field(bits="24..=31")]
    precision: u8,
    #[packed_field(bits="32..=63")]
    root_delay: u32,
    #[packed_field(bits="64..=95")]
    root_dispersion: u32,
    #[packed_field(bits="96..=127")]
    reference_id: u32,
    #[packed_field(bits="128..=159")]
    reference_timestamp: u32,
    #[packed_field(bits="160..=191")]
    reference_timestamp_fraction: u32,
    #[packed_field(bits="192..=223")]
    original_timestamp: u32,
    #[packed_field(bits="224..=255")]
    original_timestamp_fraction: u32,
    #[packed_field(bits="256..=287")]
    receive_timestamp: u32,
    #[packed_field(bits="288..=319")]
    receive_timestamp_fraction: u32,
    #[packed_field(bits="320..=351")]
    transmit_timestamp: u32,
    #[packed_field(bits="352..=383")]
    transmit_timestamp_fraction: u32,
}

fn make_init_pack() -> NTPPacket{
    NTPPacket {
        leap_indicator: 0,
        version_number: 3,      // NTP version
        mode: 3,                // client mode
        stratum: 0,
        poll: 0,
        precision: 0,
        root_delay: 0,
        root_dispersion: 0,
        reference_id: 0,
        reference_timestamp: 0,
        reference_timestamp_fraction: 0,
        original_timestamp: 0,
        original_timestamp_fraction: 0,
        receive_timestamp: 0,
        receive_timestamp_fraction: 0,
        transmit_timestamp: 0,
        transmit_timestamp_fraction: 0,
    }
}

fn show_date(timestamp: u32) {
    use std::process::Command;
    use std::string::String;
    let output = Command::new("date")
            .args(&["-d", &format!("@{}", timestamp)])
            .output()
            .expect("failed to execute process");
    println!("{}", String::from_utf8((output.stdout)).unwrap());
}

fn main() {
    let addr = "172.98.193.44:123";
    let socket = UdpSocket::bind("0.0.0.0:4555").expect("can not bind socket");
    socket.send_to(&make_init_pack().pack(), addr);
    let mut buf = [0u8; 48];
    let (length, addr) = socket.recv_from(&mut buf)
                               .expect("Didn't receive data");
    let ntppacket = NTPPacket::unpack(&buf).expect("Can't upack receive NTP replay");
    let timestamp = ntppacket.receive_timestamp - 2208988800;
    println!("{:#?}", ntppacket);
    println!("{}", timestamp);
    show_date(timestamp);
}
