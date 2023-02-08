#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

static TESTS: &[&str] = &[
    "ch2b_hello_world\0",
    "ch2b_power_3\0",
    "ch2b_power_5\0",
    "ch2b_power_7\0",
    "ch3b_yield0\0",
    "ch3b_yield1\0",
    "ch3b_yield2\0",
    "ch3b_sleep\0",
    "ch3b_sleep1\0",
    "ch4b_sbrk\0",
    "ch5b_forktest_simple\0",
    "ch5b_forktest\0",
    "ch5b_forktest2\0",
    "ch6b_filetest_simple\0",
    "ch6b_cat\0",
    "ch7b_sig_simple\0",
    "ch7b_sig_simple2\0",
    "ch7b_pipetest\0",
    "ch7b_pipe_large_test\0",
];

use user_lib::{spawn, waitpid};

/// 辅助测例，运行所有其他测例。

#[no_mangle]
pub fn main() -> i32 {
    for test in TESTS {
        println!("Usertests: Running {}", test);
        let pid = spawn(*test);
        let mut xstate: i32 = Default::default();
        let wait_pid = waitpid(pid as usize, &mut xstate);
        assert_eq!(pid, wait_pid);
        println!(
            "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
            test, pid, xstate
        );
    }
    println!("ch7 Usertests passed!");
    0
}
