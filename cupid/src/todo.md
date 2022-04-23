# Todo

## Variables

- [x] Declaration
- [x] Assignment
- [x] Immutable
- [ ] Deep immutable
- [x] Rework grammar

## Type system

- [ ] Type declaration
  - [ ] Dictionary-style
  - [ ] List-style
  - [ ] Nested
  - [x] Alias
- [ ] Enums
- [ ] Struct declaration/impl?
- [ ] Maybe types
- [ ] Map types
  - [ ] `dict (string, int)`
  - [x] `array (int)`
- [ ] Generics
- [ ] Type casting

### Type checker

- [x] Assignment type checking
- [ ] Deep assignment type checking

## Operators

- [x] Exponent
- [x] Modulus
- [x] Operator assignment
  - [x] `x++`
  - [x] `x--`
- [ ] Compare data structures
- [x] Logical and
- [x] Logical or
- [ ] Negation

## Functions

- [x] Anonymous functions
- [x] Block functions
- [ ] Function chaining
- [ ] Closed scope
- [x] Return statement
- [ ] Keyword args
- [x] Typed parameters
- [ ] Return type
- [ ] Callbacks
- [ ] Default values (allow fewer/skipped args)
- [x] No arguments
- [ ] Call immediately

## Blocks

- [x] If blocks
- [x] Else blocks
- [x] Else if blocks

## Loops

- [x] While loop
- [x] For..in loop
- [ ] Indeterminate loop
- [ ] Named loops
- [ ] Break statements
  - [x] `break`
  - [x] `break (return_value)`
  - [ ] `break identifier(return_value)`
  - [x] Continue

## Data structures

- [x] Array
- [x] Dictionary
- [ ] Tuples (keywords)
- [ ] Range
  - [x] Numbers
  - [ ] Step
  - [ ] Characters
- [x] Property access
- [x] Property assignment
- [ ] Lightweight array that isn't a map
- [ ] Add/remove properties
- [ ] Property chaining
- [ ] Self keyword
  - [x] Reference inner properties
  - [ ] Mutate inner properties

## Scoping

- [ ] Named scopes
- [x] Simple block scopes `{ # can access outer scope }`
- [x] Boxed scopes `box { # cannot access outer scope }`
- [ ] No global scope?

## Features

- [ ] De-structuring
- [ ] Pattern matching
- [ ] Array slices using range syntax e.g. `my_array[0..5]`
  - [ ] Include negative numbers
- [ ] Variable shadowing
- [ ] Template strings `'my favorite number is {{ 30 + 7 }}'`
- [ ] Escape keywords like Rusts `r#type` (only better ...)

## Bugfixes

- [ ] Something is wrong with groups in grammar files
- [ ] An empty map `[]` could be a dict or a list or anything- type inference?

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
- [ ] Lookbehind (for array)

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
