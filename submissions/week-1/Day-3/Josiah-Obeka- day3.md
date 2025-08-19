#CHAPTER 6 (STRUCTS)


##5.1 Defining and Instantiating a struct

Struct are just like tuples but with them you define all the values and their types. they are more flexible.
structs aallows for liberty to update the content of a new struct from a previous instance usingthe struct update syntax (..) and it usually comes last just to specify that it applies to all the remaining values in the struct instance. The rules of ownership still apply when certain types are moved using the struct update syntax.

##5.2 An Example Programming Using Structs

using brorrowed structs in a function is to ensure that the struct instance does not move the field values.
in order to enable us print an instance of a truct we need to use #[derive(Debug)] just before the struct definition. 
we need to also use {:#?} instead of {:?} in the println! string in other to format the rint statement to print the struct instance.

##5.3 Method Syntax

Methods are uaually defined within a atruct, enum or trait and they always take the “self” parameter as the first parameter.
Most method can take ownership of self, borrow it mutably and also borrow it immutably,
when printing the method, with parentheses, Rust knows we mean the method the entire method, but when we don’t use parentheses, Rust knows we mean the field within the method. Structs allow you to define custom types that are meaningful within your specific domain. They help you group related pieces of data together in a clear and organized way by giving each field a name. Using impl blocks, you can define functions tied to your custom type. When these functions operate on an instance of the struct (using self), they're called methods, and they define the behavior your struct instances can perform.
 


#CHAPTER 6 (ENUMS)


##6.1 Defining and enum
Enum provides a way to capture all the possible set of values. 
We can attach structs to each variant in the enum, but instead of that we can attach each each type while declaring the enum, this allows us to define each variant and even assign it to a variable. a struct variant can take any type of data(even a struct)

##6.2 The match flow construct

Rust’s match is a powerful control flow tool that lets us compare values against different patterns and run code depending on which pattern matches. It works like a coin sorting machine the value “drops” into the first matching pattern.
It also works with enums like Option<T>, and it forces you to handle all possible cases, which helps prevent bugs. Each match arm has a pattern and some code to run if that pattern matches.
Patterns can be simple, like 3 or none, or more complex, where we also bind inner values. You can also use a catch-all pattern (_) when you want to match everything else, and even ignore the value completely.
Match expressions return values, and when used well, they make our code clearer and safer

##6.3 Concise coltrol flow with if let and let else

Rust’s if let is a cleaner way to handle values that match a single pattern, especially when using enums like Option or Result. It saves us from writing full match blocks with unnecessary _ => () arms.
We can also combine if let with an else to run code when the value doesn’t match. This is handy when you want to focus on the happy path and handle exceptions more cleanly.
For even more clarity, Rust offers let...else, which lets you pull values out of patterns and immediately return or handle the error if the pattern doesn’t match. This keeps your main logic clean and easy to follow.
In short
* Use if let when you care about just one match case.
* Use if let ... else to add fallback logic.
* Use let...else for even cleaner, early-return logic when destructuring values.
