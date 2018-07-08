////////////////////////////////////////
// no std
////////////////////////////////////////

#![cfg_attr(feature = "no_std", feature(custom_attribute))]
#![cfg_attr(feature = "no_std", feature(panic_implementation))]
#![cfg_attr(feature = "no_std", feature(lang_items))]
#![cfg_attr(feature = "no_std", no_std)]    // don't link Rust standard library
#![cfg_attr(feature = "no_std", no_main)]   // disable all Rust-level entry points

#[cfg(feature = "no_std")]
extern crate libc;

#[cfg(feature = "no_std")]
use core::panic::PanicInfo;

#[cfg(feature = "no_std")]
#[lang = "panic_impl"] fn panic_impl(_: &PanicInfo) -> ! { loop {} }


////////////////////////////////////////
// NTP structure
////////////////////////////////////////

extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

pub mod packet;

use packed_struct::prelude::*;
use packet::NTPPacket;

////////////////////////////////////////
// UDP socket
////////////////////////////////////////

// #[cfg(feature = "no_std")]
// use smoltcp::socket::UdpSocket;
#[cfg(not(feature = "no_std"))]
use std::net::UdpSocket;


////////////////////////////////////////
// utils
////////////////////////////////////////

fn show_date(timestamp: u32) {
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

fn get_timestamp() -> u32 {
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


////////////////////////////////////////
// no std
////////////////////////////////////////

#[cfg(feature = "no_std")]
#[no_mangle]    // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}

////////////////////////////////////////
// std
////////////////////////////////////////

#[cfg(not(feature = "no_std"))]
fn main() {
    let timestamp = get_timestamp();
    show_date(timestamp);
}
