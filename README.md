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
- A pointer is a value that describes a location in memory.
- The value that pointer points to is called **pointee** .

> Heap is a separate region of memory where data can live indefinitely. Heap data is not tied to a spcific stack frame. Rust provides a construct called `Box` for putting data on the heap.

> **Box Deallocation priciple** : If a variable owns a box, when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory.
> Box are used by Rust data structures like Vec, String, and HashMap to hold a variable number of elements.

- Heap data can only be accessed through its current owner, not a previous owner.

### REFERENCES AND BORROWING

- A **reference** is a kind of pointer. References are non-owning pointers because they do not own the data they point to.

- Aliasing is accessing same data through different variables.
- The macro `vec!` creates a vector with the elements between the brackets.

> **Pointer Safety Principle** : Data should never be aliased and mutated at the same time.

> Immutable References are read-only( also called **shared references**). They allow aliasing but disallow mutation.
> Mutable References : Temporarily provide mutable access to data without moving it. (also called **unique references**).

- _Safe_ - Mutable references allow mutation but prevent aliasing. The borrowed path becomes temporarily unusable, so effectively not an alias
- _Useful_ - The mutable reference itself cannot be reassigned to a different mutable reference.

- Data Must Outlive All Of Its References

> The `F` permission : Expected whenever an expression uses an input reference(like &strings[0]), or returns an output reference(like `return s_ref`).

- 1. Input/Output references are treated differently than references within a function body
- 2. Rust uses a different mechanism, the `F` permission, to check hte safety of those references.

> `Rc::clone` : clones a pointer to `s` and not the data itself. At runtime, the `Rc` checks when the last `Rc` pointing to data has been dropped, and then deallocates the data.

> For resolving a reference return to a Stack : How long should my String live? Who should be in charge of deallocating it ?

- **It is very rare for Rust functions to take ownership of heap-owning data structures like `Vec` and `String`**

- In general, writing Rust functions is a careful balance of asking for the _right_ level of permissions.

- Copying a `String` copies a pointer to heap data, Copying an i32 does not. In technical terms, Rust says that the type `i32` implements the `Copy` trait, while `String` does not implement `Copy`.

- **If a value does not own heap data, then it can be copied without a move. Except for mutalbe references. For example &mut i32 is not a copyable type.**

### The Slice Type

- Lets us reference a contiguous sequence of elements in a collection rather than the whole collection.
- It is a non-owning pointer.
- Slices are pointers with metadata. Metadata is the length of the slice.

> A String is a vector of bytes (Vec\<u8>\), which contains a length `len` and a buffer `buf` that has a pointer `ptr` and a capacity `cap`.

- `s.as_bytes()` produces an immutable reference to `s`.

- #### A little Recap

  - Boxes are pointers owning data on the heap
  - References are now-owning pointers
  - Local variables can hold either data or pointers
  - Slices are a special kind of reference that refer to a contiguous sequence of data in memory.
  - A move of a variable with a non-copyable type( like Box<T> or String) requires the **RO** permissions, and the move eliminates all permissions on the variable

  - What is ok and what is not ok ->

    - printing an immutable reference is ok
    - mutating an immutable reference is not ok
    - mutating the immutably borrowed data is not ok
    - moving data out of the reference is not ok
    - mutating a mutable references is ok
    - accessing the mutably borrowed data is not ok

  - Some undefined behaviour
    - Immutable borrows remove the `W` permission to avoid `` `use-after-free` : where freed memory is read or written``
    - `` `double-free` : where memory is freed twice``. Dereferences of references to non-copyable data do not have the `O` permission to avoid double-frees
    - The `unsafe` feature doesn't completely disable the borrow checker, but rather enables the use of specific unsafe features like raw pointers.

### Structs

- Note that the entire instane of a struct should be marked mutable.
- Unit like structs: `struct StructName;`
  - They are useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself.
- Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.
- `Display` trait is for presenting values to an end-user while `Debug` is for developers' internal use.

  - Methods

    - declare them with `fn` keyword and a name, can have parameters and a return value, they contain some code that's run when the method is called from somewhere else.
    - Some differences from functions :
      - defined within a context of a struct( or an enum or a trait object)
      - their first parameter is always `self`, representing the instance of the struct the method is being called on.
      - _method syntax_ : add a dot followed by the method name, parentheses, and any arguments
      - `&self` is short for `self: &Self`.
      - Within an `impl` block, the type `Self` is an alias for the type that the `impl` block is for.
      - Methods can take ownership of `self`, borrow `self` immutably, or borrow `self` mutably, just as they can any other parameter.
    - we can choose to give a method the same name as one of the struct's fields.
    - When we follow `type.method` with parentheses, Rust knows we mean the method, and when we don't use parentheses, Rust know we mean the field with that name.
    - _Getters_
      - Often, when we give a method the same name as a field we want it to only return the value in the field and do nothing else. Such methods are called `getters`.
      - Getters are useful because we can make the field private but the method public ,thus enabling read-only access to that field.
    - Methods can only be implemented for types(e.g. impl Point), not variables( like p)

  - _Associated Functions_
    - Functions that don't have `self` as their first parameter( and thus are not methods) because they don't need an instance of the type to work with.
    - Asstd funcs. that aren't methods are often used for constructors that will return a new instance of the struct. Often called `new`.
    - Associated function is namespaced by the struct like `Rectangle::square(3)`.
    - The `::` syntax is used for both associated function and namespaces created by modules.

### Enums

- Define a type by enumerating its possible variants
- The Option Enum
  - encodes the scenario in which a value could be something or it could be nothing.
  ```
  enum Option<T> {
      None,
      Some(T),
  }
  ```
  - `<T>` is a generic type parameter. It means that `Some` variant of the `Option` enum can hold one piece of data of any type, and that each concrete type that gets used in place of `T` makes the overall `Option<T>` type a different type.

### The Module System

- It includes :
  - **Packages** : Lets us build, test, and share crates
  - **Crates** : Tree of modules that produces a library or executable
  - **Modules and use** : Lets us control the organization, scope and privacy of paths
  - **Paths** : A way of naming an item, such as a struct, function, or module
- **Crates**
  - smallest amout of code that the Rust compiler considers at a time.
  - can contain modules, which may be defined in other files that get compiled with the crate.
  - "crate" is interchangeably used with the general programming concept of a "library".
- A package can contain as many binary crates, but at most only one library crate. It must contain contain at least one crate.

- Modules Cheat Sheet
  - src/lib.rs -> library crate and src/main.rs -> binary crate
  - **declare new modules** in the crate root file, e.g. - `mod garden`
    - Compiler looks for module's code in :
      - inline, within curly brackets that replace the semicolon followin `mod garden`
      - file garden.rs
      - src/garden/mod.rs
  - declare submodules in any file other than the crate root.
    - declare `mod vegetables;` src/garden.rs
      - search among curly brackets...... src/garden/vegetables.rs and ..... src/garden/vegetables/mod.rs
  - Paths to code in modules -> `crate::garden::vegetables::Asparagus`
  - To make a module public, declare it with `pub mod` instead of `mod`.
  - To make items within a public module public as well, use `pub` before their declarations.
  - The `use` keyword : write `use crate::garden::vegetables::Asparagus`, and from then on we only need to write `Asparagus` to make use of that type in the scope.

> The reason for `src/main.rs` and `src/lib.rs` being called crate roots is as follows: The contents of either of these two files form a module named `crate` at the root of the crate's module structure, knowns as `module tree`.

- When we add the `pub` keyword besides a module of fn within a module, that module or function will be accessible only if the parent module is also accessible.
- We can construct
- If we make a struct public, all of it's individual fields should be made public too
- Instead, if we make an enum public, all of its variants are then public.


### Vectors
- default integer type is `i32`
- Non-copyable types cannot be moved out of a vector by indexing
- Only methods such as `vec::remove` permit moving out of a vector.
- The important detail : Iterators contain a pointer to data within the vector.

### String and &str
- Both `String` and string slices are UTF-8 encoded.
- Updating a String 
    - we can use `+` operator, or `format!` macro to concatenate `String` values.
    - append to a `String` with `push_str`. It takes a string slice, so no ownership taken
    - the `push` method takes a single character as a parameter and adds it to the `String`.
- A `String` is a wrapper over a `Vec<u8>`
- The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes.
- `&str` is a promise that the byte sequence it points to will always be valid UTF-8. Therefore a programmer who wants to e.g. print out an `&str` never needs to check if it is valid, or worry about accidentally interpreting an invalid string.

### HashMap
- `HashMap<K, V>` : stores a mapping of keys of type `K` to values of type `V` using a _hashing_ function.
- All the keys must have same type and all values must have same type.
- and Ownership
    - Types that implement `Copy` trait like `i32`, values are copied into the hash map
    - Owned values like `String`, values will be moved and the hash map will be the owner of those values.