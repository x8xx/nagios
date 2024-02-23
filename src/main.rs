#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(nagios::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    nagios::println!("Hello NagiOS");
    nagios::init();

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    loop {}
}



#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    nagios::test_panic_handler(info);
}




#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
