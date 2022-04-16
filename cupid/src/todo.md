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
  - [ ] Nested
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
- [ ] No arguments
- [ ] Call immediately

## Blocks

- [x] If blocks
- [x] Else blocks
- [x] Else if blocks

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
- [x] Property assignment
- [ ] Lightweight array that isn't a map
- [ ] Add/remove properties
- [ ] Property chaining

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
- [ ] Template strings `'my favorite number is {{ 30 + 7 }}'`

## Bugfixes

- [ ] Something is wrong with groups in grammar files

## Builtin library

- [ ] String functions/properties
  - [ ] Length
  - [ ] Contains
  - [ ] Replace/replace all
- [ ] Map functions

## Standard library

- [ ] Random
- [ ] Rust-like iterators

## Meta

- [ ] Benchmarking performance
- [ ] Optimization
  - [ ] `String` to `Cow`
- [ ] Documentation

### Error handling

- [ ] Differentiate between errors and warnings
- [ ] Report errors before compiling

## Parser

- [ ] Error handling by using branching grammar syntax
  - [ ] Write custom common errors associated with rules
  - [ ] `recover!` macro
- [ ] Inline start/end comments

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
