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
- unrecoverable - symptoms of bugs.
    - `panic!` - stops execution when the program encounters an unrecoverable error.
    - these panics print a failure message, unwind, clean up the stack, and quit.
    - add `panic = 'abort'` to appropriate `[profile]` sections to end program without cleaning up.
        - e.g. 
        ```
        [profile.release]
        panic = 'abort'
        ```
    