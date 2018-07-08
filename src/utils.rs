////////////////////////////////////////
// UDP socket
////////////////////////////////////////

// #[cfg(feature = "no_std")]
// use smoltcp::socket::UdpSocket;
#[cfg(not(feature = "no_std"))]
use std::net::UdpSocket;


////////////////////////////////////////
// NTP packet
////////////////////////////////////////

use packed_struct::prelude::*;
use packet::NTPPacket;


////////////////////////////////////////
// utils
////////////////////////////////////////

pub fn show_date(timestamp: u32) {
    #[cfg(feature = "no_std")]
    {
    }
    #[cfg(not(feature = "no_std"))]
    {
        use std::process::Command;
        use std::string::String;
        let output = Command::new("date")
                .args(&["-d", &format!("@{}", timestamp)])
                .output()
                .expect("failed to execute process");
        println!("{}", String::from_utf8(output.stdout).unwrap());
    }
}

pub fn get_timestamp() -> u32 {
    let addr = "172.98.193.44:123";
    #[cfg(feature = "no_std")]
    {
        NTPPacket::new().pack();
        0
    }
    #[cfg(not(feature = "no_std"))]
    {
        let init_ntppacket = NTPPacket::new().pack();
        let socket = UdpSocket::bind("0.0.0.0:4555").expect("can not bind socket");
        socket.send_to(&init_ntppacket, addr);
        let mut buf = [0u8; 48];
        let (length, addr) = socket.recv_from(&mut buf)
                                   .expect("Didn't receive data");
        let ntppacket = NTPPacket::unpack(&buf).expect("Can't upack receive NTP replay");
        let timestamp = ntppacket.receive_timestamp - 2208988800;
        println!("{:#?}", ntppacket);
        println!("{}", timestamp);
        timestamp
    }
}

pub fn get_timestamp_and_show() {
    let timestamp = get_timestamp();
    show_date(timestamp);
}
