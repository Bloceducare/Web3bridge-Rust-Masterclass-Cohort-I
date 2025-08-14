# Rust Programming Notes

## Chapter 3

Chapter three talks about the basics of rust which includes the data types. So in rust we have two categories of datatypes which include scalar and compound data types.

Scalar datatypes are of fixed size, while compound datatypes consists of multiple datatypes into one. It also talks about functions and function signatures and how parameters are typed and then how we can also return data. To return data we need to define the return type.

Then how comments work and control flow which involves how the rust program runs, or branches based of some set conditions. We have the if, while, for, loop control flows in rust.

## Chapter 4

Ownership in rust is a way rust help to manage memory efficiently and how it works is by working with scopes and data structures like stack and heaps. So storing data on the stack is easier and its also easy to retrieve, but storing on the heap which is used for data of dynamic or no known length at compile time and a pointer to the data location is then pushed to the stack which is used to trace the data and use.

So in such situation if a variable is assigned a value and another is assigned that variable, the initial variable is moved to the other variable and does not exist again. So if we try to do something with that variable the compiler will give errors. We say that ownership has been transferred to the other variable.

But there are ways to work around this with referencing and borrowing where we do not have to move the data but just store a reference to that data. So when we assign a variable to another variable using the reference symbol &, the data is not moved but a reference to the location of the data which we call a pointer is stored in that variable and can be used to get the data in that location. The term of using references is called borrowing.

## Questions

- What is the slice type?
- What if the variable referenced from is deleted, how then does it work?