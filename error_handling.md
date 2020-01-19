## Unrecoverable Errors

- use the `panic!` macro
- on execution, the programm will print a failure message, unwind and clean up
  the stack, and then quit
- unwinding/clean up takes a lot of work, alternative use `panic = 'abort'` in
  the `Cargo.toml` file

  ```toml
  [profile.release]
  panic = 'abort'
  ```

## Recoverable Errors

- use the `Result` enum
- `unwrap()` is a helper function which panics with the `Err` message
- `expect()` is a helper function which allows a prefix for the `panic!` message

## Error Propagation

- shortcut `?` operator
- value with the `?` shortcut go through the `from` function to convert the
  error
- `?` eliminates a lot of boilerplate
