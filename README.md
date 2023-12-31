# dsll - Doubly Sorted Linked List
dsll is a Rust library that provides a safe and fast implementation of a thread-safe and concurrent sorted doubly linked list (DSLL). It is designed to offer efficient insertion while ensuring thread safety and consistency in a concurrent environment.

# Features
- Sorted Order: Elements are maintained in ascending order based on their natural ordering (comparable trait).

- Thread Safety: The DSLL is designed to be safe for concurrent access from multiple threads. It uses fine-grained locking to ensure that operations do not interfere with each other.

- Fast Performance: This data structure is optimized for high-performance operations, such as insertions, making it suitable for applications with demanding performance requirements.

# Usage
To use dsll in your Rust project, simply add it as a dependency in your project using Cargo:

```
cargo add dsll
```

