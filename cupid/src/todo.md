# Todo

## Variables

- [x] Declaration
- [x] Assignment
- [x] Immutable
- [ ] Deep immutable
- [ ] Assignment type checking

## Operators

- [ ] Exponent
- [ ] Operator assignment
- [ ] Compare data structures

## Functions

- [x] Anonymous functions
- [x] Block functions
- [ ] Function chaining
- [ ] Closed scope
- [ ] Return statement
- [ ] Keyword args

## Blocks

- [x] If blocks
- [x] Else blocks
- [ ] Else if blocks

## Loops

- [x] While loop
- [x] For..in loop
- [ ] Indeterminate loop
- [ ] Break
- [ ] Named loops

## Data structures

- [x] Array
- [x] Dictionary
- [ ] Tuples (keywords)
- [ ] Range
- [x] Property access
- [ ] Property assignment
- [ ] Lightweight array that isn't a map

## Features

- [ ] De-structuring
- [ ] Pattern matching
- [ ] Array slices using range syntax e.g. `my_array[0..5]`
  - [ ] Include negative numbers
- [ ] Rust-like iterator features
- [ ] Modules/import/export

## Standard library

- [ ] Random

## Meta

- [ ] Benchmarking performance
- [ ] Error handling: create error nodes in tree
  - [ ] A node is "poisoned" if it has an error- all other nodes that interact
        with that node are poisoned as well
- [ ] LSP
- [ ] Optimization
  - [ ] `String` to `Cow`
- [ ] Command line tools
  - [ ] Testing
  - [ ] Running files
    - [ ] "cupid play my_file.cupid"
  - [ ] Package manager??
