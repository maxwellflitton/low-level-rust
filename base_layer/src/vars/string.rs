use alloc::alloc::{alloc, dealloc, Layout};
use core::ptr::NonNull;

// SimpleString has 3 fields: ptr to allocated memory, capacity; amount of memory allocated, current length of string
pub struct SimpleString {
    ptr: NonNull<u8>,
    capacity: usize,
    length: usize,
}

impl SimpleString {

    // function for creating a SimpleString from a string slice    
    pub fn from(s: &str) -> Self {
        let bytes = s.as_bytes(); // converts string to sequence of bytes
        let capacity = bytes.len(); // finds length of sequence of bytes
        let layout = Layout::array::<u8>(capacity).unwrap(); // creates a layout for an array of bytes (u8) with specific capacity as defined before
        let ptr = unsafe { alloc(layout) }; // allocates memory for the array of bytes
        // altered below to use .unwrap() to call custom panic handler rather than using .unwrap_or_else(|| unsafe { libc::abort() }) for consistency with above code
        let ptr = NonNull::new(ptr).unwrap();

        // copies contents of input string into allocated memory
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr.as_ptr(), bytes.len());
        }

        // returns a SimpleString struct with the allocated memory, capacity, and length of the input string
        SimpleString {
            ptr,
            capacity,
            length: bytes.len(),
        }
    }


// converts from String to &str
    pub fn as_str(&self) -> &str {
        unsafe {
            // self.ptr.as_ptr() obtains the underlying raw pointer
            let slice = core::slice::from_raw_parts(self.ptr.as_ptr(), self.length);
            core::str::from_utf8_unchecked(slice)
        }
    }
}


// define behaviour for when simple string goes out of scope
impl Drop for SimpleString {
    fn drop(&mut self) {
        let layout = Layout::array::<u8>(self.capacity).unwrap();
        unsafe {
            dealloc(self.ptr.as_ptr(), layout);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // test SimpleString::from and SimpleString::as_str
    #[test]
    fn test_simple_string_from_str() {
        let s = SimpleString::from("Hello, World!");
        assert_eq!(s.as_str(), "Hello, World!");
        assert_eq!(s.capacity, 13);
        assert_eq!(s.length, 13);
    }

    #[test]
    fn test_empty_string_from_str() {
        let s = SimpleString::from("");
        assert_eq!(s.as_str(), "");
        assert_eq!(s.capacity, 0);
        assert_eq!(s.length, 0);
    }

    #[test]
    fn test_long_string_from_str() {
        let long_string = "abcd".repeat(100000);
        let s = SimpleString::from(long_string.as_str());
        assert_eq!(s.as_str(), "abcd".repeat(100000));
        assert_eq!(s.capacity, 100000*4);
        assert_eq!(s.length, 100000*4);
    }

    #[test]
    fn test_special_string_from_str() {
        let s = SimpleString::from("Hello 😃");
        assert_eq!(s.as_str(), "Hello 😃");
        assert_eq!(s.capacity, 10);
        assert_eq!(s.length, 10);
    }

    // use cargo test --features count-allocations to run - try to fix this:
    #[cfg(feature = "count_allocations")]
    #[test]
    fn test_memory_allocation_deallocation() {
        let memory = allocation_counter::measure(|| {
            SimpleString::from("Hello, World!");
        });
        // track global memory and ensure deallocation
    }

    // ensure drop works with capacity of 0 (no memory leaks)
    // track global memory allocation/deallocation with multiple drops
    // check capacity = allocated memory

    // remember to mention altering panic handler code in main.rs in PR

}