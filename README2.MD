### ERROR HANDLING 
- recoverable- report to the user and retry the operation
    - handled through  `Result<T, E>`
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



### Notables
- `fs::read_to_string`; opens the file, creates a new `String`, reads the contents of the file, puts the contents into that `String`, and returns it.
- We're only allowed to use the `?` operator in a function that returns `Result`, `Option`, or another type that implements `FromResidual` 
- `lines` method on string slice returns an iterator over the line in the string. `next` on this iterator will give the first value from the iterator.
- `chars` on a string slice to get an iterator of its characters. calling `last` on this iterator returns last item in the iterator.
- `Box<dyn Error>>` means "any kind of error"