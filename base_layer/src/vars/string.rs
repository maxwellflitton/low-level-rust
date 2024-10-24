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
        let ptr = NonNull::new(ptr).unwrap_or_else(|| unsafe { libc::abort() });

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
    }

    #[test]
    fn test_empty_string() {
        let s = SimpleString::from("");
        assert_eq!(s.as_str(), "");
        assert_eq!(s.capacity, 0);
        assert_eq!(s.length, 0);
    }

    #[test]
    fn test_empty_string() {
        let s = SimpleString::from("");
        assert_eq!(s.as_str(), "");
        assert_eq!(s.capacity, 0);
        assert_eq!(s.length, 0);
    }

    // other unit test ideas:
    // long string
    // non-ascii? - but this wont work because of implementation?
    // test drop
    // 



    // remember to uncomment panic handler code in main.rs

}
