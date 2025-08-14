
## Summary of what i understand.
- rust is sttically type lanaguage which mean at any even point in time, rust must know the variable type at compile time, which men every value in a rust program is of certain data type.
- main function is the entry point of rust program
- every function which is define with some argument, thoes argument must be of certain data type
- control flow let you execute a block of code base on the condition, this is similar to every other programming langauge
- in rust every value has only one owner at a time, which mean every point in time that another function want to use that that value, it either take ownership or borrow the value 
- you can borrow value via reference which can be done two ways 
- immutable reference: this mean you are borrowing the value without the ability to change it
- mutable refeence: this allow you to borrow with the ability to change the value
- mutable refeence bring us to one very important rule of borrowing in rust.
- At any given time, you can have either one mutable reference or any number of immutable reference.


## List what you do **not** understand.
- String slice
- slice as parameters
