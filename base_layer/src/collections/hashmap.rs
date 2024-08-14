use std::ptr::null_mut;
use std::ffi::CString;
use libc::{malloc, free, c_void};

struct KeyValuePair {
    key: CString,
    value: i32,
    next: *mut KeyValuePair,
}

struct HashMap {
    buckets: *mut *mut KeyValuePair,
    capacity: usize,
}

impl HashMap {
    pub fn new(capacity: usize) -> Self {
        let buckets = unsafe {
            let size = capacity * std::mem::size_of::<*mut KeyValuePair>();
            let ptr = malloc(size) as *mut *mut KeyValuePair;
            if ptr.is_null() {
                libc::abort();
            }
            std::ptr::write_bytes(ptr, 0, capacity);
            ptr
        };
        HashMap { buckets, capacity }
    }

    pub fn insert(&mut self, key: &str, value: i32) {
        let c_key = CString::new(key).unwrap();
        let bucket_index = self.hash(&c_key) % self.capacity;
        let mut current = unsafe { *self.buckets.add(bucket_index) };

        while !current.is_null() {
            unsafe {
                if (*current).key == c_key {
                    (*current).value = value;
                    return;
                }
                current = (*current).next;
            }
        }

        let new_pair = unsafe {
            let new_pair = malloc(std::mem::size_of::<KeyValuePair>()) as *mut KeyValuePair;
            if new_pair.is_null() {
                libc::abort();
            }
            (*new_pair).key = c_key;
            (*new_pair).value = value;
            (*new_pair).next = *self.buckets.add(bucket_index);
            new_pair
        };

        unsafe {
            *self.buckets.add(bucket_index) = new_pair;
        }
    }

    pub fn get(&self, key: &str) -> Option<i32> {
        let c_key = CString::new(key).unwrap();
        let bucket_index = self.hash(&c_key) % self.capacity;
        let mut current = unsafe { *self.buckets.add(bucket_index) };

        while !current.is_null() {
            unsafe {
                if (*current).key == c_key {
                    return Some((*current).value);
                }
                current = (*current).next;
            }
        }
        None
    }

    pub fn delete(&mut self, key: &str) {
        let c_key = CString::new(key).unwrap();
        let bucket_index = self.hash(&c_key) % self.capacity;
        let mut current = unsafe { *self.buckets.add(bucket_index) };
        let mut prev: *mut KeyValuePair = null_mut();

        while !current.is_null() {
            unsafe {
                if (*current).key == c_key {
                    if !prev.is_null() {
                        (*prev).next = (*current).next;
                    } else {
                        *self.buckets.add(bucket_index) = (*current).next;
                    }
                    free(current as *mut c_void);
                    return;
                }
                prev = current;
                current = (*current).next;
            }
        }
    }

    fn hash(&self, key: &CString) -> usize {
        let bytes = key.as_bytes_with_nul();
        let mut hash = 5381;
        for byte in bytes {
            hash = (hash.wrapping_shl(5)).wrapping_add(hash).wrapping_add(*byte as usize);
        }
        hash
    }
}

impl Drop for HashMap {
    fn drop(&mut self) {
        for i in 0..self.capacity {
            let mut current = unsafe { *self.buckets.add(i) };
            while !current.is_null() {
                unsafe {
                    let next = (*current).next;
                    free(current as *mut c_void);
                    current = next;
                }
            }
        }
        unsafe {
            free(self.buckets as *mut c_void);
        }
    }
}
