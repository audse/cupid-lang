# Todo

## Variables

- [x] Declaration
- [x] Assignment
- [x] Immutable
- [x] Deep immutable
- [x] Rework grammar
- [ ] Add const/let for type inference

## Type system

- [x] Type declaration
  - [x] Product (struct)
  - [x] Sum (enum)
  - [x] Nested
  - [x] Alias
- [ ] Maybe types
- [x] Map types
  - [x] `dict (string, int)`
  - [x] `array (int)`
- [x] Generics
- [ ] Type casting
- [ ] First-class types
  - [ ] Pass as values/args
  - [x] Log
  - [ ] Builtin functions e.g. typeof
- [ ] Sum type variants
- [ ] Tagged sum type variants
- [ ] Gradual typing

### Traits
```
type my_struct = [
  int my_data,
  fun [int] do_something
]

use default with my_struct [
  do_something: self => self.my_data
]

trait [t] add [
  fun [t] add 
]

use add with my_struct [
  add: self, my_struct other => self.my_data + other
]
```
- [ ] Type implementations
  - [x] Declare `use` block
  - [ ] Call associated functions on any struct instance

### Type checker

- [x] Assignment type checking
  - [x] Approximate
- [x] Deep assignment type checking
- [ ] Property assignment check

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
- [ ] Operator overloading

## Functions

- [x] Anonymous functions
- [x] Block functions
- [ ] Function chaining
- [ ] Closed scope
- [x] Return statement
- [ ] Keyword args
- [x] Typed parameters
- [x] Return type
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

## Values

- [x] Array
- [x] Dictionary
- [ ] Tuples (keywords)
- [ ] Range
  - [ ] Numbers
  - [ ] Step
  - [ ] Characters
- [x] Property access
- [x] Property assignment
- [x] Lightweight array that isn't a map
- [ ] Add/remove properties
- [ ] Property chaining (needs to be left recursive)
- [ ] Self keyword
  - [x] Reference inner properties
  - [ ] Mutate inner properties
- [ ] Number/string types
  - [ ] Irrational numbers
  - [ ] UTF-8, 16, etc
  - [ ] Signed/unsigned numbers

## Scoping

- [ ] Named scopes
- [x] Simple block scopes `{ # can access outer scope }`
- [x] Boxed scopes `box { # cannot access outer scope }`
- [ ] No global scope?
- [ ] Inject standard library stuff into boxed scopes

## Features

- [ ] De-structuring
- [ ] Pattern matching
- [ ] Array slices using range syntax e.g. `my_array[0..5]`
  - [ ] Include negative numbers
- [x] Variable shadowing
- [ ] Template strings `'my favorite number is {{ 30 + 7 }}'`
- [ ] Escape keywords like Rusts `r#type` (only better ...)
- [ ] Method overloading

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
- [ ] Constants such as PI

## Meta

- [ ] Benchmarking performance

### Optimization

- [ ] `String` to `Cow`
- [ ] Cut down on clones- use `Rc`
- [ ] Instead of reassigning whole symbol, mutate symbol value


### Error handling

- [ ] Differentiate between errors and warnings
- [ ] Report errors before compiling
- [ ] Function param & return type mismatches
- [ ] Add more error handling in the parsing phase

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

### Online playground
- [ ] Set up web assembly
- [ ] Create basic code editor (CodeMirror)
- [ ] Create basic syntax highlighting
- [ ] Host on Github pages

### Documentation
- [ ] Style guide
