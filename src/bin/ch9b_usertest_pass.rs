#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const TESTS: &[&str] = &[
    "ch2b_hello_world\0",
    "ch2b_power_3\0",
];

const TEST_NUM: usize = TESTS.len();

use user_lib::{exec, fork, waitpid};

#[no_mangle]
pub fn main() -> i32 {
    let mut pids = [0; TEST_NUM];
    for (i, &test) in TESTS.iter().enumerate() {
        println!("Usertests: Running {}", test);
        let pid = fork();
        if pid == 0 {
            println!("Child: {}",test);
            return 0;
            //exec(&*test, &[core::ptr::null::<u8>()]);
            //panic!("unreachable!");
        } else {
            println!("Parent: fork {}: {}", test, pid);
            pids[i] = pid;
        }
    }
    let mut xstate: i32 = Default::default();
    for (i, &test) in TESTS.iter().enumerate() {
        println!("Parent: wait {}",pids[i]);
        let wait_pid = waitpid(pids[i] as usize, &mut xstate);
        assert_eq!(pids[i], wait_pid);
        println!(
            "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
            test, pids[i], xstate
        );
    }
    println!("Basic usertests passed!");
    0
}
