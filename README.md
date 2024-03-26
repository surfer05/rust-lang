` rustup doc` for the documentation
`println!` when calling a rust macro and `println` when calling a function

#### running a rust file without cargo or any other tool or package manager :
- create a.rs
- run `rustc a.rs` in terminal
- run `./a` in terminal

- `cargo check` to check that the code compiles but not producing an executable.

- cargo build --release 

- string literals are string slices
- `binary crate` - which is executable
- `library crate` - contains code that is intended to be used in other programs, and can't be executed on its own
- `trim` - this method on a string eliminates any whitespace at the beginning and end.
- Rust has two primitive compound types : Tuples and Arrays
- NO fsize data type : only f32 and f64
- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value.
- Expressions do not include ending semicolons
- statements don’t evaluate to a value, which is expressed by (), the unit type.

### Memory is the space where data is stored during the execution of a program. 
### Ownership is discipline of _pointer_ management.
 
- A frame is a mapping from variables to values within a single scope, such as a function.
- A pointer is a value that  describes a location in memory.
- The value that pointer points to is called __pointee__ .

> Heap is a separate region of memory where data can live indefinitely. Heap data is not tied to a spcific stack frame. Rust provides a construct called `Box` for putting data on the heap.

> __Box Deallocation priciple__ : If a variable owns a box, when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory.
> Box are used by Rust data structures like Vec, String, and HashMap to hold a variable number of elements.

- Heap data can only be accessed through its current owner, not a previous owner.
