# learn-rust

## Data Race

a _data race_ is similar to a race condition and happens when these three behaviors occur:

1. Two or more pointers access the same data at the same time.
2. At least one of the pointers is being used to write to the data.
3. There's no mechanism ensuring that changes by one pointer are visible to other pointers.

## Dangling References

A _dangling reference_ occurs when a reference that is being used refers to data that has been deallocated.

In languages with pointers:

- It's easy to erroneously create a dangling pointer by freeing memory while preserving a pointer to that memory
- A dangling pointer references a location in memory that may have been given to someone else

In Rust:

- The compiler guarantees that references will never be dangling references
- If you have a reference to data, the compiler ensures the data won't go out of scope before the reference does
