use alloc::alloc::{alloc, dealloc, Layout};
use core::ptr::NonNull;


pub struct SimpleVec {
    ptr: NonNull<u8>,
    capacity: usize,
    pub length: usize,
}

impl SimpleVec {
    pub fn new(capacity: usize) -> Self {
        // get a vector that is capable of holding `capacity` elements
        // this is slighly different from libc malloc as this can prevent overflows
        // it also does the spacing for you. For instance, if you were writing a 4 byte integer
        // in C you would have to move the pointer by 4 bytes to write the next integer where
        // as the Layout does this for you
        let layout = Layout::array::<u8>(capacity).unwrap();
        // get the pointer to the allocated memory
        let ptr: *mut u8 = unsafe { alloc(layout) };
        // if the pointer is null, abort the program
        let ptr: NonNull<u8> = NonNull::new(ptr).unwrap_or_else(|| unsafe { libc::abort() });
        SimpleVec { ptr, capacity, length: 0 }
    }

    pub fn push(&mut self, value: u8) {
        if self.length >= self.capacity {
            // Reallocate with more capacity with a new layout
            let new_capacity = self.capacity * 2;
            let new_layout = Layout::array::<u8>(new_capacity).unwrap();
            let new_ptr = unsafe { alloc(new_layout) }; // Note: This is simplified; realloc would be used here.
            let new_ptr = NonNull::new(new_ptr).unwrap_or_else(|| unsafe { libc::abort() });
            unsafe {
                // copy over the data and deallocate the old memory
                core::ptr::copy_nonoverlapping(self.ptr.as_ptr(), new_ptr.as_ptr(), self.length);
                dealloc(self.ptr.as_ptr(), Layout::array::<u8>(self.capacity).unwrap());
            }
            // update the pointer and capacity
            self.ptr = new_ptr;
            self.capacity = new_capacity;
        }
        // write the value to the end of the vector in the memory
        unsafe {
            core::ptr::write(self.ptr.as_ptr().add(self.length), value);
        }
        self.length += 1;
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        if index < self.length {
            unsafe { Some(core::ptr::read(self.ptr.as_ptr().add(index))) }
        } else {
            None
        }
    }
}

impl Drop for SimpleVec {
    fn drop(&mut self) {
        let layout = Layout::array::<u8>(self.capacity).unwrap();
        unsafe {
            dealloc(self.ptr.as_ptr(), layout);
        }
    }
}