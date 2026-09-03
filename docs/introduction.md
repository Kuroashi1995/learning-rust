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
- [Crates](#crates)
- References
- [Mutability](#mutability)
- Variations
- Enums
- Traits
- [Shadowing](#shadowing)

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

### Shadowing
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
