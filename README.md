# learn-rust

## Data Race

a _data race_ is similar to a race condition and happens when these three behaviors occur:

1. Two or more pointers access the same data at the same time.
2. At least one of the pointers is being used to write to the data.
3. There's no mechanism ensuring that changes by one pointer are visible to other pointers.
