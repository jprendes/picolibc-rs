#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

use picolibc::host::LinuxHost;
use picolibc::time::SystemTime;
use picolibc::{println, thread_local};

#[picolibc::host]
static HOST: LinuxHost = LinuxHost;

#[picolibc::main]
fn main() {
    println!("Hello from Rust!");

    test_allocations();
    test_tls();
    test_time();
}

/// Do a bunch of allocations to force the allocator to request
/// more memory from the host several times.
fn test_allocations() {
    // You can see the allocator's behavior by tracing calls to mmap with strace:
    // $ strace -e trace=mmap target/x86_64-unknown-none/debug/picolibc-demo
    let mut v = vec![];
    let mut allocated = 0;
    for _ in 0..16 {
        // The allocator requests memory from the host in 64 KiB chunks,
        // You should see a first mmap call for the first 2 iterations,
        // another one for the next 4 iterations, and so on.
        let mut vec = Vec::with_capacity(31 * 1024);
        vec.extend([1u8, 2, 3, 4, 5]);
        allocated += vec.capacity();
        println!("Bytes allocated: 0x{allocated:x}");
        v.push(vec);
    }
}

// Test that thread-local storage works by reading and writing a TLS variable.
fn test_tls() {
    thread_local! {
        static TLS_VAR: RefCell<Vec<u8>> = RefCell::new(vec![1,2,3,4]);
    }

    // The first call should be initialized with the initial value
    TLS_VAR.with_borrow(|v| {
        println!("TLS_VAR = {v:?}");
    });

    // Mutate the TLS variable
    TLS_VAR.with_borrow_mut(|v| {
        v.push(5);
    });

    // This call should see the mutated value
    TLS_VAR.with_borrow(|v| {
        println!("TLS_VAR = {v:?}");
    });
}

fn test_time() {
    let time = SystemTime::now();
    println!("Current time: {time}");
}
