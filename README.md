# Midas Lang

A coding language that I wanted to make while learning how to use Rust.
I am referencing the book "Crafting Interpreters" and a tutorial online loosely. It pretty much goes off of a Java implementation so am doing my own Rust-based version of it.
This is a work in progress, and I hope to have basic functionality soon.

---
## To Run via CLI

```
cargo run
```
---
## Mini-Documentation
- [Print](#print)
---
### Print
- **Description**: Prints value to the console.
- **Usage**: 
    ```text
    print(3 + 6) -> 9
    ```
- **Parameters**: 
  - *v*: The literal value to be printed to the console.
  
- **Return Value**: Returns a value that gets printed to console.
---
### FEATURES:
- Math!
- Some basic syntax/error suggestions
- Truthiness and equality
- String concatenation
- Variables & Printing
- A cool funny looking dude in the prompt
---
### IN PROGRESS:
- States/Statements/Vars
---
# TODO:
### Scanner - [x]
Basic right now, make better in future
**Want to change keywords later to camel case / diff names
### Expression Tree - [x]
Small optimizations to be made, use of
unwrapping/clone should be limited
### Literal Values - [x]
Some small things here and there to look back and fix
### Parser - [x]
Small bugfixes needed such as '(' parenthesis and  
when to panic vs when no to
### Interpreter - [x]
Add more cases / line numbers in error msgs
### Statements - [ ]


