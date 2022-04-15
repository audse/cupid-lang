# Todo

## Variables

- [x] Declaration
- [x] Assignment
- [x] Immutable
- [ ] Deep immutable
- [x] Assignment type checking
- [ ] Deep assignment type checking
- [ ] Rework grammar

## Type system

- [ ] Type declaration
  - [x] Dictionary-style
  - [ ] List-style
- [ ] Enums
- [ ] Struct declaration/impl?
- [ ] Maybe types

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
- [ ] Type hints
- [ ] Callbacks
- [ ] Default values

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
- [ ] Add/remove properties

## Scoping

- [ ] Named scopes
- [ ] Simple block scopes `{ # can access outer scope }`
- [ ] Boxed scopes `box { # cannot access outer scope }`
- [ ] Break statements
  - [ ] `break (return_value)`
  - [ ] `break identifier(return_value)`

## Features

- [ ] De-structuring
- [ ] Pattern matching
- [ ] Array slices using range syntax e.g. `my_array[0..5]`
  - [ ] Include negative numbers
- [ ] Variable shadowing

## Bugfixes

- [ ] Something is wrong with groups in grammar files

## Standard library

- [ ] Random
- [ ] Rust-like iterators

## Meta

- [ ] Benchmarking performance
- [ ] Optimization
  - [ ] `String` to `Cow`

### Error handling

- [ ] Differentiate between errors and warnings
- [ ] Report errors before compiling

## Ideas the future

- [ ] Module import & export
- [ ] Language server
- [ ] Command line tools
  - [ ] Testing
  - [ ] Running files `cupid play my_file.cupid`
  - [ ] Package manager
- [ ] Formatter
- [ ] Linter
- [ ] Vscode extension
- [ ] Nova extension?
- [ ] Online playground
