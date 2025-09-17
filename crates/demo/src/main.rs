#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;

use picolibc::host::LinuxHost;
use picolibc::println;

#[picolibc::host]
static HOST: LinuxHost = LinuxHost;

#[picolibc::main]
fn main() {
    println!("Hello from Rust!");
    let vec = vec![1, 2, 3, 4, 5];
    println!("Vector: {:?}", vec);
}
