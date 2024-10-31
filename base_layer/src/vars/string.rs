use alloc::alloc::{alloc, dealloc, Layout};
use core::ptr::NonNull;

pub struct SimpleString {
    ptr: NonNull<u8>,
    capacity: usize,
    length: usize,
}

impl SimpleString {

    pub fn from(s: &str) -> Self {
        let bytes = s.as_bytes();
        let capacity = bytes.len();
        let layout = Layout::array::<u8>(capacity).unwrap();
        let ptr = unsafe { alloc(layout) }; 
        let ptr = NonNull::new(ptr).unwrap();

        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr.as_ptr(), bytes.len());
        }

        SimpleString {
            ptr,
            capacity,
            length: bytes.len(),
        }
    }


    pub fn as_str(&self) -> &str {
        unsafe {
            let slice = core::slice::from_raw_parts(self.ptr.as_ptr(), self.length);
            core::str::from_utf8_unchecked(slice)
        }
    }
}


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

    #[test]
    fn test_simple_string() {
        let s = SimpleString::from("Hello, World!");
        assert_eq!(s.as_str(), "Hello, World!");
        assert_eq!(s.capacity, 13);
        assert_eq!(s.length, 13);
    }

    #[test]
    fn test_empty_string() {
        let s = SimpleString::from("");
        assert_eq!(s.as_str(), "");
        assert_eq!(s.capacity, 0);
        assert_eq!(s.length, 0);
    }

    #[test]
    fn test_long_string() {
        let long_string = "abcd".repeat(1000000);
        let s = SimpleString::from(long_string.as_str());
        assert_eq!(s.as_str(), "abcd".repeat(1000000));
        assert_eq!(s.capacity, 4000000);
        assert_eq!(s.length, 4000000);
    }

    #[test]
    fn test_multi_byte_char_string() {
        let s = SimpleString::from("Hello 😃");
        assert_eq!(s.as_str(), "Hello 😃");
        assert_eq!(s.capacity, 10);
        assert_eq!(s.length, 10);
    }

    #[test]
    fn test_string_with_null_character() {
        let s = SimpleString::from("Hello\0World");
        assert_eq!(s.as_str(), "Hello\0World");
        assert_eq!(s.capacity, 11);
        assert_eq!(s.length, 11);
    }

    #[test]
    fn test_explicit_drop() {
    let s = SimpleString::from("Test drop");
    drop(s);
    }
    // Test passes if there’s no panic, suggesting successful deallocation, but more comprehensive tests require a memory tracker or external tool like Valgrind.

}