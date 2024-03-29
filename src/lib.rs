#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
pub mod serial;
pub mod interrupts;
pub mod test;
// use core::panic::PanicInfo;


pub fn init() {
    interrupts::init_idt();
}


// #[cfg(test)]
// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     init();
//     test_main();
//     loop {}
// }

// #[cfg(test)]
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     test_panic_handler(info);
// }


/*
 * ==================================
 *           custom test
 * ==================================
 */
// pub trait Testable {
//     fn run(&self) -> ();
// }

// impl<T> Testable for T
// where
//     T: Fn(),
// {
//     fn run(&self) {
//         print!("{}...\t", core::any::type_name::<T>());
//         self();
//         println!("[ok]");
//     }
// }



// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[repr(u32)]
// pub enum QemuExitCode {
//     Success = 0x10,
//     Failed = 0x11,
// }

// pub fn exit_qemu(exit_code: QemuExitCode) {
//     use x86_64::instructions::port::Port;

//     unsafe {
//         let mut port = Port::new(0xf4);
//         port.write(exit_code as u32);
//     }
// }


// pub fn test_runner(tests: &[&dyn Testable]) {
//     println!("Running {} tests", tests.len());
//     for test in tests {
//         test.run();
//     }
//     exit_qemu(QemuExitCode::Success);
// }

// pub fn test_panic_handler(info: &PanicInfo) -> ! {
//     println!("[failed]\n");
//     println!("Error: {}\n", info);
//     exit_qemu(QemuExitCode::Failed);
//     loop {}
// }

