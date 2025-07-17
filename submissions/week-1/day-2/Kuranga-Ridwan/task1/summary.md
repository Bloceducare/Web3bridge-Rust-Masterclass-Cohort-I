```python


The guessing game program asks for user input and it process that input
we need to bring the io input/output library into scope for the inputs of the users
the stdin function from the io module allows us to handle user input
the line .read_line(&mut guess) calls the read_line method on the standard input handle to get input from the user.
we have the cmp Ordering that matches the user input with whether its too small , too big or equal to the guessed number
.expect crashes the programm with a specific type of error
The & indicates that this argument is a reference, which gives us a way to let multiple parts of our code access one piece of data without needing to copy that data into memory multiple times.
the rand crate is a library crate, which contains code that is intended to be used in other programs and 
can’t be executed on its own.
we use rand to generate random numbers 
The parse method on strings converts a string to another type
variables are immutable by default
mutability can only occur with the mut keyword
constants must be in snake cases
shadowing is different from mutability
we have scalar types and compound types of Data types

```