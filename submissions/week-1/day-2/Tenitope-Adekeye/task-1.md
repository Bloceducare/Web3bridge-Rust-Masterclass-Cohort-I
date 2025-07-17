
## Things To Note:
    - Curly braces define a new scope, but have access to variables from parent scopes.
    - signed & unsigned integers have the same range, but unsigned integers covers more on the positive side, since it has no negative side to cover.
    - Handling Overflows:
        Wrap in all modes with the wrapping_* methods, such as wrapping_add.
        Return the None value if there is overflow with the checked_* methods.
        Return the value and a Boolean indicating whether there was overflow with the overflowing_* methods.
        Saturate at the value’s minimum or maximum values with the saturating_* methods.
    - Rust’s char type is four bytes in size and represents a Unicode scalar value.
    - One of the main advantages and purpose of the ownership model in Rust is to prevent the two most prevalent classes of bugs in C and C++,
      the first being dangling pointers, and the second being use-after-free bugs, the implementation of the "Drop" trait also helps prevent memory leaks, I think.
    - The slice type for a string is stored in the global section of the binary, either .exe or elf.