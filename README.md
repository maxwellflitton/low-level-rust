# low-level-rust
a series of exercises for low level Rust with no standard library

# Custom Rust Base Layer Checklist

## Core Data Structures
- [x] Vector
- [x] HashMap
- [x] String
- [ ] VecDeque
- [ ] HashSet
- [ ] BTreeMap
- [ ] BTreeSet
- [ ] LinkedList
- [ ] BinaryHeap

## Memory Management
- [ ] Allocator: Build a basic memory allocator (e.g., bump allocator, free list, etc.).
- [ ] Smart Pointers: Implement basic versions of smart pointers like `Box`, `Rc`, and `RefCell`.
- [ ] Manual Memory Handling: Implement utilities for manual memory management, such as `malloc`, `free`, and memory safety features.

## Concurrency Primitives
- [ ] Mutex: Implement a basic mutex for thread safety.
- [ ] Spinlock: Implement a simple spinlock for lightweight locking in concurrent programming.
- [ ] Channels: Create a minimal version of channels for inter-thread communication.

## File and IO Operations
- [ ] File Handling: Implement low-level file operations (e.g., open, read, write) using system calls directly through `libc`.
- [ ] Streams: Build basic input/output stream abstractions.
- [ ] Buffered I/O: Implement buffered readers and writers for efficient data processing.

## Basic Network Utilities
- [ ] Socket: Implement a basic socket abstraction to work with networking.
- [ ] TCP/UDP Abstractions: Build minimal TCP and UDP client/server abstractions for network communication.

## Error Handling
- [ ] Custom Error Types: Create a base error handling mechanism, similar to Rust's `Result` and `Option` types.
- [ ] Panic Handling: Implement a simple panic handler that works without the standard library.

## Collections
- [ ] Linked List: Implement a basic singly and doubly linked list.
- [ ] Binary Tree: Create a binary search tree, which will help you explore recursion and memory layout management.
- [ ] Set: Implement a basic set using a hash-based or tree-based approach.

## Math and Algorithms
- [ ] Basic Math Functions: Implement fundamental math functions like `sqrt`, `log`, `exp`, etc.
- [ ] Sorting Algorithms: Implement basic sorting algorithms like quicksort, mergesort, and heapsort.

## Low-Level System Interaction
- [ ] Syscalls: Build abstractions for making system calls, essential for interacting with the operating system directly.
- [ ] Environment Access: Implement functions to access environment variables and command-line arguments.

## Type System Utilities
- [ ] Option and Result: Implement basic versions of `Option` and `Result` to handle optional and error-prone operations safely.
- [ ] Traits: Build custom traits (interfaces) and experiment with trait objects and generic programming.

## Threading and Task Management
- [ ] Threads: Implement a minimal threading system, possibly starting with a simple thread creation and management API.
- [ ] Coroutine/Async: Experiment with a basic coroutine or async runtime.

## Time and Timers
- [ ] Time Handling: Implement functions for working with time, like getting the current time, sleeping for a duration, etc.
- [ ] Timers: Build a simple timer API that could be used for time-based operations.

## Foreign Function Interface (FFI)
- [ ] C Bindings: Experiment with creating FFI bindings to interact with C libraries directly.

## Debugging Utilities
- [ ] Logging: Implement a basic logging system to trace and debug the operations in your custom layer.
- [ ] Assertions: Build a simple assertion library for testing your components.

## Custom Compiler Built-ins
- [ ] Intrinsics: Implement basic compiler intrinsics that might be missing without the standard library, such as atomic operations or specific CPU instructions.
