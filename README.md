# Midas Lang

A coding language that I wanted to make while learning how to use Rust.
I am referencing the book "Crafting Interpreters" and a tutorial online loosely. It pretty much goes off of a Java implementation so am doing my own Rust-based version of it.
This is a work in progress, and I hope to have basic functionality soon.


## To Run via CLI

```
cargo run
```

### FEATURES:
- Math!
- Some basic syntax/error suggestions
- Truthiness and equality
- String concatenation
- A cool funny looking dude in the prompt

# TODO:

### Scanner - [x]
Basic right now, make better in future
**Want to change keywords later to camel case / diff names
### Expression Tree - [x]
Small optimizations to be made, use of
unwrapping should be limited
### Parser - [x]
Small bugfixes needed such as '(' parenthesis and  
when to panic vs when no to
### Interpretor - [ ]
Move to seperate file? Right now just in expr
Add more cases / line numbers in error msgs
Format error message of Literal types better (ex: TrueVal to 'true')


