#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(nagios::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    nagios::test::test_panic_handler(info)
}
