//! # Note
//! This is a 'cp' program based on Rust.
//!
//! Usage:
//!
//! ```bash
//! rust-cp [file_src] [file_dst]
//! ```
//!
//! @auhtor:    Mrbanana
//! @date:      2021-7-26
//! @license:   The MIT License
//!
//! # Knowledge you will learn
//! - How to read and write a file in Rust
//! - How to get argv[] from enviroment variable

use std::env;
use std::fs::*;
use std::io::*;

fn main() {
    // 1. Get argv[] from env
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 3 {
        eprintln!("Usage: rust-cp [file_src] [file_dst]");
        std::process::exit(1);
    }

    // 2. Open read-file and write-file
    let mut fd_read = File::open(argv[1].as_str()).expect("Cannot open source file!");
    let mut fd_write = File::create(argv[2].as_str()).expect("Cannot open destination file!");

    // 3. Copy file
    let mut buffer = [0u8; 4096];
    loop {
        let read_bytes = fd_read.read(&mut buffer).expect("Error in reading file.");
        fd_write
            .write(&buffer[..read_bytes])
            .expect("Error in writing file.");

        if read_bytes < buffer.len() {
            break;
        }
    }

    println!("{}\t >>> Copy Done!{}", "\x1b[32m", "\x1b[0m");
}
