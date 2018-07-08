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


////////////////////////////////////////
// modules
////////////////////////////////////////

pub mod packet;
pub mod utils;


////////////////////////////////////////
// no std
////////////////////////////////////////

#[cfg(feature = "no_std")]
#[no_mangle]    // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    // utils::get_timestamp_and_show();
    loop {}
}


////////////////////////////////////////
// std
////////////////////////////////////////

#[cfg(not(feature = "no_std"))]
fn main() {
    utils::get_timestamp_and_show();
}
