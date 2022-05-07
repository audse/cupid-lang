# Todo

## Variables

-   [x] Declaration
-   [x] Assignment
-   [x] Immutable
-   [x] Deep immutable
-   [x] Rework grammar
-   [ ] Add const/let for type inference

## Type system

-   [x] Type declaration
    -   [x] Product (struct)
    -   [x] Sum (enum)
    -   [x] Nested
    -   [x] Alias
-   [ ] Maybe types
-   [x] Map types
    -   [x] `dict (string, int)`
    -   [x] `array (int)`
-   [ ] Generics
    -   [x] In type declaration
    -   [x] In use blocks
    -   [ ] In functions
    -   [ ] Trait bounds
-   [ ] Type casting
    -   [x] Primitives
    -   [ ] Array
    -   [ ] Map
    -   [ ] Generics
-   [ ] First-class types
    -   [ ] Pass as values/args
    -   [x] Log
    -   [x] `istype`
-   [ ] Sum type variants
-   [ ] Tagged sum type variants
-   [ ] Gradual typing
-   [ ] Compound implementations
-   [ ] Param type hints in fun signature

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

-   [ ] Type implementations

    -   [ ] Declare `use` block
        -   [x] Primitives
        -   [ ] Array
        -   [ ] Map
        -   [ ] Function (this can be how to implement things like call, bind, decorators, etc)
        -   [x] Struct
        -   [x] Alias
        -   [x] Sum
    -   [x] Call associated functions on any struct instance
    -   [ ] Require implementation of all functions without defaults
    -   [ ] Make sure no extra functions are added
    -   [ ] Implement different variations with different type args...

        e.g. `use [bool] into with int` and `use [dec] into with int`

-   [x] Require `self` on functions that use self
-   [ ] Require `mut self`

### Type checker

-   [x] Assignment type checking
    -   [x] Approximate
-   [x] Deep assignment type checking
-   [x] Property assignment check
-   [ ] Check that all elements in array/map are same type
-   [ ] No approximate type checking for structs when using `istype`

```
type person = [
  string name,
]
map [string, string] someone = [
  name: 'Jane Doe'
]
someone istype person # should be false
```

-   [ ] `uses` operator for traits e.g. `int uses add`

## Operators

-   [x] Exponent
-   [x] Modulus
-   [x] Operator assignment
-   [ ] Compare data structures
-   [x] Logical and
-   [x] Logical or
-   [ ] Negation
-   [ ] Operator overloading
-   [x] Type of
-   [ ] Use trait implementations instead of simple value checking

## Functions

-   [x] Anonymous functions
-   [x] Block functions
-   [ ] Function chaining
-   [ ] Closed scope
-   [x] Return statement
-   [ ] Keyword args
-   [x] Typed parameters
-   [x] Return type
-   [x] Callbacks
-   [ ] Default values (allow fewer/skipped args)
-   [x] No arguments
-   [ ] Call immediately
-   [ ] Closures need some help..capture scope inside function body

## Blocks

-   [x] If blocks
-   [x] Else blocks
-   [x] Else if blocks

## Loops

-   [x] While loop
-   [x] For..in loop
-   [ ] Indeterminate loop
-   [ ] Named loops
-   [ ] Break statements
    -   [x] `break`
    -   [x] `break (return_value)`
    -   [ ] `break identifier(return_value)`
    -   [x] Continue

## Values

-   [x] Array
-   [x] Dictionary
-   [ ] Tuples (keywords)
-   [ ] Range
    -   [x] Numbers
    -   [ ] Step
    -   [ ] Characters
-   [x] Property access
-   [x] Property assignment
-   [x] Lightweight array that isn't a map
-   [ ] Add/remove properties
-   [ ] Property chaining (needs to be left recursive)
-   [ ] Self keyword
    -   [x] Reference inner properties
    -   [ ] Mutate inner properties
-   [ ] Number/string types
    -   [ ] Irrational numbers
    -   [ ] UTF-8, 16, etc
    -   [ ] Signed/unsigned numbers

## Scoping

-   [ ] Named scopes
-   [x] Simple block scopes `{ # can access outer scope }`
-   [x] Boxed scopes `box { # cannot access outer scope }`
-   [ ] No global scope?
-   [ ] Inject standard library stuff into boxed scopes

## Features

-   [ ] De-structuring
-   [ ] Pattern matching
-   [ ] Array slices using range syntax e.g. `my_array[0..5]`
    -   [ ] Include negative numbers
-   [x] Variable shadowing
-   [ ] Template strings `'my favorite number is {{ 30 + 7 }}'`
-   [ ] Escape keywords like Rusts `r#type` (only better ...)
-   [ ] Method overloading

## Bugfixes

-   [ ] Something is wrong with groups in grammar files
-   [ ] An empty map `[]` could be a dict or a list or anything- type inference?
-   [ ] If you use a primitive type where generics should go, it acts as a generic. Should be an error
-   [ ] Should be able to log property functions without calling them (e.g. `log (myint.sq)`)
-   [x] Is `log`/`log_line` working?
    -   [x] Doesn't work in function bodies

## Builtin library

-   [ ] String functions/properties
    -   [ ] Length
    -   [ ] Contains
    -   [ ] Replace/replace all
-   [ ] Map functions

## Standard library

-   [ ] Random
-   [ ] Rust-like iterators
-   [ ] Constants such as PI

## Meta

-   [ ] Benchmarking performance
-   [ ] Debugger

### Optimization

-   [ ] `String` to `Cow`
-   [ ] Cut down on clones- use `Rc`
-   [ ] Instead of reassigning whole symbol, mutate symbol value
-   [ ] Make parsing faster
    -   [ ] Branching rather than repeats
    -   [ ] Organize infix operators better

### Error handling

-   [ ] Differentiate between errors and warnings
-   [ ] Report errors before compiling
-   [ ] Function param & return type mismatches
-   [ ] Add more error handling in the parsing phase
-   [ ] Change errors as values to Result/Ok

## Parser

-   [ ] Error handling by using branching grammar syntax
    -   [ ] Write custom common errors associated with rules
    -   [ ] `recover!` macro
-   [ ] Inline start/end comments
-   [ ] Lookbehind (for array)

## Ideas the future

-   [ ] Module import & export
-   [ ] Language server
-   [ ] Command line tools
    -   [ ] Testing
    -   [ ] Running files `cupid play my_file.cupid`
    -   [ ] Package manager
-   [ ] Formatter
-   [ ] Linter
-   [ ] Vscode extension
-   [ ] Nova extension?

### Online playground

-   [x] Set up web assembly
-   [x] Create basic code editor (CodeMirror)
-   [x] Create basic syntax highlighting
-   [ ] Host on Github pages
-   [ ] Create some example files
-   [ ] Autocomplete
-   [ ] Save in local storage
-   [x] Load standard library
-   [ ] Better error viewing/debugging
-   [ ] Make mobile version
-   [ ] View options
-   [ ] Builtin documentation
-   [ ] Simulate multiple tabs/files

### Documentation

-   [ ] Overview
-   [ ] Standard library
    -   [ ] Integer
        -   [x] Built in methods
        -   [ ] Trait implementations
    -   [ ] Char
    -   [ ] Decimal
    -   [ ] String
    -   [ ] Array
    -   [ ] Map
    -   [ ] Traits
-   [ ] Style guide
