#![feature(const_checked_int_methods)]
#![feature(trace_macros)]
#![feature(proc_macro_hygiene)]
#![recursion_limit = "256"]

use bitflags::bitflags;
use bitmask::bitmask;

bitflags! {
    struct Fields: u32 {
        const A = bitmask!(..=2);
        const B = bitmask!(2..9);
        const C = bitmask!(9..32);
        const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
    }
}

fn main() {
    println!("A: {:#x}", Fields::A.bits());
    println!("B: {:#x}", Fields::B.bits());
    println!("C: {:#x}", Fields::C.bits());
}
