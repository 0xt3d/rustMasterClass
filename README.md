# Rust-Lang Master Class
Rust is a modern system programming language focused on performance, safety, and concurrency. It accomplishes these goals without having a garbage collector, making it a useful language for a number of use cases other languages aren’t good at. Its syntax is similar to C++, but Rust offers better memory safety while maintaining high performance.

It's a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. It is graced with the feature of “memory safety without garbage collection”, an attribute that makes Rust one of its kind. Memory safety is ensuring that software, while accessing the system’s memory, is not causing any leaks, or dangling pointers. In Rust, memory safety is accomplished through a system of ownership with a set of rules that the compiler checks at compile time. This system eliminates the need of garbage collection or manual memory management, hence ensuring swift execution of software along with a safer memory environment. This memory management feature in Rust even provides concurrent programming guaranteeing thread safety with options for shared and mutable state access that makes it harder to cause thread unsafety.

## Veriables

In Rust, variables are declared using the let keyword. They are immutable by default, which means once a value is bound to a variable, it cannot be changed. If you want to make a variable mutable, the mut keyword is used. So, if you wanted to declare a mutable variable x and assign it the value 5, you would write let mut x = 5;. Variables can also be patterned. By default in Rust, variables are block-scoped. Rust also supports several types of variable attributes.


- helloworld.rs
  - Learned to basics of the rust