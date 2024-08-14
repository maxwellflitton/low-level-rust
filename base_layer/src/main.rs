#![no_main]
#![no_std]

extern crate alloc;
mod collections;
mod vars;
mod utils;
mod memory;

use core::panic::PanicInfo;
use libc::c_int;

use memory::allocators::LibcMalloc;
use collections::vector::SimpleVec;
use vars::string::SimpleString;


#[global_allocator]
static GLOBAL: LibcMalloc = LibcMalloc;

// personality and unwind has been given with the following:
// https://github.com/rust-lang/rust/issues/106864

#[no_mangle]
extern "C" fn rust_eh_personality() {}

#[allow(non_snake_case)]
#[no_mangle]
extern "C" fn _Unwind_Resume() {}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { libc::abort() }
}


#[no_mangle]
pub extern "C" fn main() {

    let mut vec = SimpleVec::new(4);

    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);

    for i in 0..vec.length {
        unsafe {
            let val = vec.get(i).unwrap();
            let integer = val as c_int;

            libc::printf("Value: %d\n\0".as_ptr() as *const i8, integer);
        }
    }

    for value in &vec {
        unsafe {
            libc::printf("Value: %d\n\0".as_ptr() as *const i8, value as c_int);
        }
    }

    unsafe {
        libc::printf("Vector Length: %d\n\0".as_ptr() as *const i8, vec.length as c_int);
    }

    let s = SimpleString::from("Hello, World!");
    unsafe {
        libc::printf("String: %s\n\0".as_ptr() as *const i8, s.as_str().as_ptr());
    }
}
