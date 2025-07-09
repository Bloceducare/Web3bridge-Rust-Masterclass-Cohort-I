# chapter 3
## ownership rules
1  each value in rust has a single owner
2  therr can be only one owner at a time
3  when the owner goes out of scope the value is dropped
## Stack vs Heap
- stack:stores data with known compile size
- used for dynamic data e. string

## borrowing
- immutable borrow :can read but not modify
mutable borrow: can be modified only one allowed at a time
# chpter 4
- recoverable :uses result <T,E> (e.g, file not found).
-recoverable : used for operations that may suceeed(OK) or fail(ERR)
