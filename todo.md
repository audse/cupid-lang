# Todo

## Variables

- [ ] Explicit references (mutable state)
- [ ] Store variables in vec, not hashmap?

## Type system

- [ ] Trait bounds
- [ ] Type casting
	- [ ] Primitives
	- [ ] Array
	- [ ] Generics
	- [ ] Round decimal instead of clipping
- [ ] First-class types
	- [ ] Pass as values/args
- [ ] Sum type variants
- [ ] Tagged sum type variants
- [ ] Gradual typing
- [ ] Compound implementations
- [ ] Param type hints in fun signature
- [ ] Resolve types before other symbols

### Traits

- [ ] Type implementations
	- [ ] Require implementation of all functions without defaults
	- [ ] Make sure no extra functions are added
	- [ ] Implement different variations with different type args...

		e.g. `use [bool] into with int` and `use [dec] into with int`

### Type checker

- [ ] Check that all elements in array/map are same type

```
type person = [
	string name,
]
map [string, string] someone = [
	name: 'Jane Doe'
]
someone istype person # should be false
```

- [ ] `uses` operator for traits e.g. `int uses add`

## Functions

- [ ] Function chaining
- [ ] Return statement
- [ ] Keyword args
- [ ] Default values (allow fewer/skipped args)
- [ ] Mutable params
- [ ] Operations

## Loops

- [ ] While loop
- [ ] For..in loop
- [ ] Indeterminate loop
- [ ] Named loops
- [ ] Break statements
	- [ ] `break`
	- [ ] `break (return_value)`
	- [ ] `break identifier(return_value)`
	- [ ] Continue

## Values

- [ ] Tuples (optional keywords)
- [ ] Range
- [ ] Number/string types
	- [ ] Irrational numbers
	- [ ] UTF-8, 16, etc
	- [ ] Signed/unsigned numbers

## Scoping

- [ ] Named scopes
- [ ] Replace single scope index with list of scope indices

## Features

- [ ] De-structuring
- [ ] Pattern matching
- [ ] Array slices using range syntax e.g. `my_array[0..5]`
	- [ ] Include negative numbers
- [ ] Variable shadowing
- [ ] Template strings `'my favorite number is {{ 30 + 7 }}'`
- [ ] Escape keywords like Rusts `r#type` (only better ...)
- [ ] Method overloading

### Syntactic sugar

- [ ] Simplify traits with one function like:

```
# from:
trait [t] add! = [
	fun [t] add! = self, t other => self + other
]
# to:
trait [t] add! = self, t other => self + other
```

- [ ] Apply `with` type as first generic argument if not otherwise specified

```
# from:
use [t: int] add! with int
# to:
use add! with int
# doesn't apply in the case of:
use [t: dec] add! with int
```

- [ ] Make generic brackets optional for types with only one parameter

```
# from:
fun [array [map [string, int]]]
# to:
fun array map [string, int]
```

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
- [ ] Debugger

### Optimization

- [x] `String` to `Cow`
- [ ] Cut down on clones/owned- use `Rc`
- [ ] Instead of reassigning whole symbol, mutate symbol value
- [ ] Make parsing faster
	- [ ] Branching rather than repeats
	- [ ] Organize infix operators better
- [ ] Use arena instead of passing nodes/children around directly
	- [ ] Do this in the parsing phase

### Error handling

- [ ] Differentiate between errors and warnings
- [ ] Report errors before compiling
- [ ] Function param & return type mismatches
- [ ] Add more error handling in the parsing phase
- [x] Change errors as values to Result/Ok
- [ ] Account for tabs in line/index

## Packages

- [x] Import files
- [ ] Import specific symbols from files
	- [ ] Bring in dependencies of those symbols

## Parser

- [ ] Error handling by using branching grammar syntax
	- [ ] Write custom common errors associated with rules
	- [ ] `recover!` macro
- [ ] Inline start/end comments
- [ ] Lookbehind (for array)
- [x] Functions

## Ideas the future

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

- [x] Set up web assembly
- [x] Create basic code editor (CodeMirror)
- [x] Create basic syntax highlighting
- [ ] Host on Github pages
- [ ] Create some example files
- [ ] Autocomplete
- [ ] Save in local storage
- [x] Load standard library
- [ ] Better error viewing/debugging
- [ ] Make mobile version
- [ ] View options
- [ ] Builtin documentation
- [ ] Simulate multiple tabs/files

### Documentation

- [ ] Overview
- [ ] Standard library
	- [ ] Builtin methods
		- [ ] Integer
		- [ ] Char
		- [ ] Decimal
		- [ ] String
		- [ ] Array
		- [ ] Map
	- [ ] Trait implementations
		- [ ] Integer
		- [ ] Char
		- [ ] Decimal
		- [ ] String
		- [ ] Array
		- [ ] Map
	- [ ] All Traits
- [ ] Style guide
