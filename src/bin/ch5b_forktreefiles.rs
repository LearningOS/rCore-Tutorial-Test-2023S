#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

extern crate alloc;
//use alloc::string::String;
use crate::alloc::string::ToString;

use user_lib::{close, exit, fork, getpid, open, read, sleep_blocking, write, yield_, OpenFlags};

const DEPTH: usize = 4;

fn fork_child(cur: &str, branch: char) {
    let mut next = [0u8; DEPTH + 1];
    let l = cur.len();
    if l >= DEPTH {
        return;
    }
    next[..l].copy_from_slice(cur.as_bytes());
    next[l] = branch as u8;
    if fork() == 0 {
        fork_tree(core::str::from_utf8(&next[..l + 1]).unwrap());
        yield_();
        exit(0);
    }
}

fn fork_tree(cur: &str) {
    println!("pid{}: {}", getpid(), cur);
    let test_str = "Hello, world!";
    let name="txt";
    let filea = name.to_string() + cur + "\0";
    let fd = open(filea.as_str(), OpenFlags::CREATE | OpenFlags::WRONLY);
    yield_();
    fork_child(cur, '0');
    yield_();
    assert!(fd > 0);

    let fd = fd as usize;
    write(fd, test_str.as_bytes());
    yield_();
    close(fd);
    yield_();
    fork_child(cur, '1');
    yield_();
    let fd = open(filea.as_str(), OpenFlags::RDONLY);
    yield_();
    assert!(fd > 0);
    let fd = fd as usize;
    let mut buffer = [0u8; 100];
    let read_len = read(fd, &mut buffer) as usize;
    yield_();
    close(fd);
    yield_();
    assert_eq!(test_str, core::str::from_utf8(&buffer[..read_len]).unwrap(),);

}

#[no_mangle]
pub fn main() -> i32 {
    fork_tree("");
    sleep_blocking(3000);
    0
}
