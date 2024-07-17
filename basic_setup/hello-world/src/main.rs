#![no_std]
#![no_main]

// extern crate libc;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { libc::abort() }
}

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        // Example system call: write "Hello, world!" to stdout

        // define data for the message
        let message: &[u8; 14] = b"Hello, world!\n";
        // get the pointer
        let pointer: *const u8 = message.as_ptr();
        // convert the pointer to a void pointer
        let buffer: *const libc::c_void = pointer as *const libc::c_void;
        // call the write system call
        libc::write(libc::STDOUT_FILENO, buffer, message.len());
    }
}
