### Summary 
## Ownership
- In chapter 4 first things i learnt was about the stack and heap, the three rules of ownership, then i also understood how scoping works for rust which is pretty much the same with how it works in other languages other languages. 

- Moving foward in the chapter i learnt about string and string literal and how different they are which is string literal can **not** be dynamical mutated becasue it is hardcoded and known at compile time while string (aka string::from) can be mutated with methods such as push. This difference is because of how they interact with memory (literal -> stack, normal -> heap)

- I also learnt about the concept of moving which rust implements by default to help guide against double freeing of the heap when a _copied_ variable goes out of scope.

- I learnt how clone works by copying the value allocated on the heap to another variable. 

- We can say that copy is to other types has clone is to string.

## Referencing and borrowing 
- The major thing i picked in this section are the rules of borrowing 
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.