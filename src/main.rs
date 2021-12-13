#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tuzos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use tuzos::println;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Booting TuzOS {}", VERSION);
    tuzos::init();

    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }

    // trigger a stack overflow
    // stack_overflow();

    // trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    tuzos::test_panic_handler(info)
}

#[test_case]
fn simple_test() {
    assert_eq!(1, 1);
}
