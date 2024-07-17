#![no_main]
#![no_std]

extern crate alloc;
mod allocator;
mod vector;

use core::panic::PanicInfo;
use libc::c_int;

use allocator::LibcAllocator;
use vector::SimpleVec;


#[global_allocator]
static GLOBAL: LibcAllocator = LibcAllocator;

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
}
