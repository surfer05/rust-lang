### ERROR HANDLING

- recoverable- report to the user and retry the operation
  - handled through `Result<T, E>`
  -
  ```
  enum Result<T, E> {
      Ok(T),
      Err(E),
  }
  ```
  - `T` - type of value returned in success case within the `Ok` variant.
  - `E` - type of error that will be returned in failure case within the `Err` variant.
  - The `unwrap` function returns the value inside the `Ok` if `Result` valus is the `Ok` variant. If the `Result` value is the `Err` variant, `unwrap` will call the `panic!` macro for us.
- unrecoverable - symptoms of bugs.

  - `panic!` - stops execution when the program encounters an unrecoverable error.
  - these panics print a failure message, unwind, clean up the stack, and quit.
  - add `panic = 'abort'` to appropriate `[profile]` sections to end program without cleaning up.
    - e.g.
    ```
    [profile.release]
    panic = 'abort'
    ```

- _Propagating_ Errors : When a function's implementation calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do.

### Generic Types, Traits, and Lifetimes

- `Generics` : abstract stand-ins for concrete types or other properties.
- `Traits` : to define behavior in a generic way.
- `Lifetimes`: a variety of generics that give the compiler information about how references relate to each other.
- _Monomorphization_ : turning generic code into specific code by filling in the concrete types that are used when compiled.

#### Traits

- Defines functionality a particular type has and can share with other types.
- Used to define shared behavior in an abstract way.
- similar to _interfaces_ with some differences.
- _trait bounds_ : specify that a generic type can be any type that has certain behavior.
- "orphan rule" : cannot implement an external trait for an external type, to ensure code doesn't break if two crates provide conflicting implementations.
- _Blanket Implementations_ : Implementations of trait on any type that satisfies the trait bounds. e.g.

```
impl<T : Display> ToString for T {
    // --snip--
}
```

### Validating References with Lifetimes

- Ensure that references are valid as long as we need them to be.
- _Lifetime_ : the scope for which that reference is valid.
- _Dangling References_ : cause a program to reference data other than the data it's intended to reference.

```
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

- This annotation means an instance of `ImporantExcerpt` can't outlive the reference it holds in its `part`
- Lifetimes of function or method parameters are called _input lifetimes_, and lifetimes on return values are called _output lifetimes_.

- `Lifetime Elision Rules` : Total Three rules
  - First : compiler assigns a different lifetime parameter to each lifetime in each input type.
  - Second : If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`
  - Third : If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of `self` is assigned to all output lifetime parameters.
  - Lifetime names for struct fields always need to be declared after the `impl` keyword and then used after the struct's name, because those lifetimes are part of the struct's type.
    - __The Static Lifetime__ : `'static` : the affected reference _can_ live for the entire duration of the program.
    - All string literals have `'static` lifetime




### How to Write Tests
- A test in Rust is a function that's annotated with `test` attribute.
- `Attribute` - metadata about pieces of Rust code.
- Function to test function ==> add `#[test]` on the line before `fn`

- Controlling how to run tests
    - Use `--test-threads` and number of threads we want to use to the the test binary
    - e.g. `cargo test -- --test-threads=1` tells program not to use any parallelism.
    - If we want to see printed values for passing tests as well, we use the following flag
      `--show-output`
    - Pass the name of any test function to `cargo test` to run only that test.
- Test Organization
    - The convention is create a module named `tests` in each file to contain the test functions and to annotate the module with `cfg(test)`.
    - `#[cfg(test)]` annotation on the tests module tells Rust to compile and run the test code only when you run `cargo test`, not when you run `cargo build`
    - `cfg` -> _configuration_
    - Integration tests
        - make a `tests` folder
        - make test files within this folder, and including the root crate in all the files is must.
        - Each integration test file has its own section.



### Iterators and Closures
- _Closures_ - a function-like construct you can store in a variable
- _Iterators_ - a way of processing a series of elements
- _Toilet Closures_ -
  - similar to `std::mem::drop`, i.e. a function that moves an argument and causes it to be dropped.

### Cargo and crates.io
- `cargo doc --open` is best..... for a function and _documentation comment_
- Re-exporting takes a public item in one location and makes it public in another location, as if it were defined in the other location instead.


### Smart Pointers
- data structures that act like a pointer but also have additional metadata and capabilities.
- Implement the `Deref` and `Drop` traits.
- `Deref` - allows a smart pointer struct instance behave like a reference 
-  `Drop` - customize the code that's run when an instance of the smart pointer goes out of scope.
  - `Box<T>`
    - Allow to store data on the heap rather than the stack. On the stack remains the pointer to the heap data.
    - "Indirection" means that instead of storing a value directly, we should change the data structure to store the value indirectly by storing a pointer to the value instead.
    - _Deref Coercion_ converts "a reference to a type that implements the `Deref` trait"=> "a reference to another type".
      - e.g. convert `&String` to `&str` cause `String` implements `Deref` traits such that it returns `&str`
- Rust does deref coercion when it finds types and trait implementations in three cases :
  - From `&T` to `&U` when `T : Deref<Target=U>`
  - From `&mut T` to `&mut U` when : `T : DerefMut<Target=U>`
  - From `&mut T` to `&U` when : `T: Deref<Target=U>`

### Notables

- `fs::read_to_string`; opens the file, creates a new `String`, reads the contents of the file, puts the contents into that `String`, and returns it.
- We're only allowed to use the `?` operator in a function that returns `Result`, `Option`, or another type that implements `FromResidual`
- `lines` method on string slice returns an iterator over the line in the string. `next` on this iterator will give the first value from the iterator.
- `chars` on a string slice to get an iterator of its characters. calling `last` on this iterator returns last item in the iterator.
- `Box<dyn Error>>` means "any kind of error"
