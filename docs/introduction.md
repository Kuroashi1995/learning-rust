# Introduction to Rust
It is stated that Rust is kind of a flexible programming language, closing the gap between what you can control depending on the experience you have.
Meaning that you can handle high level responsabilities if inexperienced or get into the deeper levels if you are confident in you abilities.

## Useful Tools
- It has Cargo, it's dependency manager
- It has a very verbal compiler
- It has its own formatting tool and LSP
- It is important to learn to read Rust error messages

## Important keywords
- Macros
- [Types](#types)
- [Crates](#crates)
- References
- [Mutability](#mutability)
- Variations
- Enums
- Traits
- [Shadowing](#shadowing)
- [Functions](#functions)

### Crates
This are how the packages of Rust are called, there are binary crates, which are the programs made to be ran, and crates that cannot run as standalones and are made to be part of other crate

### Mutability
By default, variables are not mutable in Rust, it is made this way to make working with concurrency easier. But it is also the reason the keyword `mut` exists, is the only way to reassign a value to an existing `let` declared variable
```rust
fn main() {
    // let x = 5; <-- Not mutable, code won't compile
    let mut x = 5;
    println!("Value is {x}")
    x = 6;
    println!("Value is now {x}")
   }
```
#### Constants
As a side note in mutability, constants do exist too in Rust, they are even more limited than the `let` declared variables, they cannot be assigned as the result of a runtime operation, they can be declared in any scope and their type needs to be anotated.
example:
```rust
const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
```

### Shadowi
The concept of shadowing is the ability to re declare a variable with the same name, "shadowing" the previous declarations of that variable. It lasts until out of scope or getting shadowed again.
example:
```rust
fn main() {
        let x = 5;
        let x = x + 1;
        {
                let x =  x * 2;
                println!("The value of x in the inner scope is {x}");
        }
        println!("The value of x is {x}");
    }
```
Another advantage of shadowing over mutability, is that we do not need to change the type of the variable when reassigning a new value to the variable. But the tradeoff is more stack memory usage.

### Types
Rust is a statically typed language, so it needs to know the types used in compile time, some can be inferred by use, other need explicit anotation otherwise compilation will fail.

There are two type classifications in Rust, Scalar Types and Compound Types.

#### Scalar Types
This are the classic primitives that we all know, but they can only represent one value
- int: (can be signed "i" or unsigned "u") like i32 for a 32-byte signed integer or u32 for a 32-byte unisgned integer
- floating point: (are allways signed) like f32 or f64
- booleans: (notation is `bool`)
- char: represents a single UTF-8 char value

#### Compound Types
This classification hold types that can store multiple values, such as:
- tuples (this can store many types, fixed len)
- arrays (can hold many values of the same type, fixed len)

### Functions
Functions in Rust are quite simple and similar to other languages, the main issue relies in the statement vs expression differentiation needed to fully grasp how functions work in Rust.
- Statements: This are instructions that perform some action and do not return a value
- Expressions: This instructions return a value

As a simple example lets look at this code:
```rust
fn main() {
        let y = 6;
}
```
In this code, the line `let y = 6;` is an statement, it assigns the value 6 to the variable `y` and returns nothing
In the other hand:
```rust
fn main() {
        let x = five()
        println!("The value of x is {x}")
}
fn five() {
        lex y = 4;
        x + 1
}
```
This code will compile, and will print "The value of x is 5", Because the last line of the five function is an expression `x + 1`. If we were to change it to `x + 1;`, that would turn it into an statement, removing the returned value and making the five function return the unit type `()`.
Functions in Rust usually evaluate to the last expression in the body of the function, the keyword `return` is used to return early from the function. Return types must be declared explicitely with `->` in the function declaration
