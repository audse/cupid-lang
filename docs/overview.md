# At a glance

Cupid is a statically typed language with a focus on readable syntax and quality-of-life features.

## 1. Primitive types

By default, Cupid includes the following data types:

### Integer `int`

```
int x = 10
int mut y = 100
```

### Character `char`

```
char some_letter = \a
char mut some_character = \&
```

### Decimal `dec`

Decimals are stored as an integer/fraction pair, not a floating point number.

This means that you can safely do equality checks without approximation.

```
dec num = 10.89
1.23 is 1.23 # true
```

### Boolean `bool`

```
bool yes = true
bool mut no = false
```

### String `string`

```
string a = 'yay cupid!'
string mut b = "cupid lang ftw!"
```

### Function `fun`

```
fun [int] add = int one, int other => one + other
```

### Array `array [t]`

```
array [int] nums = 0, 1, 2
array [char] chars = \a, \b, \c
```

### Map `map [k, v]`

```
map [string, string] name = [
	first: 'Jane',
	middle: 'C',
	last: 'Doe'
]
map [int, char] nums_as_chars = [
	0: \0,
	1: \1,
	2: \2,
]
```

### Nothing `nothing`

```
nothing my_var
```

## 2. Type system

### Define types

#### Structs/product types

Any number of named fields

```
type person = [
	int age,
	string name,
]

person jane = [
	age: 34,
	name: 'Jane Doe'
]
```

#### Alias types

Creates a wrapper for another type. Useful when the same type requires different
implementation.

```
type str = array [char]
str custom_string = \c, \u, \p, \i, \d

type year = int
```

#### Sum/union types

```
type number = [
	int,
	dec
]
```

### Implement types

```
type year = int # alias

use int {
	fun [bool] is_leap = self => {
		self % 4 is 0
	}
}

year birth_year = 1999
birth_year.is_leap() # false
```

### Type casting

```
int mynum = 10
dec newnum = mynum as dec

if mynum as bool => # do something
```

## 3. Traits

Traits are defined and implemented the same way as types.

```
type name = array [string]
trait [t] default = [
	fun [t] default,
]
use default with name {
	fun [name] default = _ => ['Jane', 'C', Doe']
}
name jane = name.default() # 'Jane', 'C', 'Doe'
```

## 4. Flow

### Blocks

Blocks are defined by braces, and can be used anywhere. The last expression in a
block will be returned, unless there is a specific return statement.

Arrow blocks `=> ..` and brace blocks `{ .. }` can be used interchangeably. The
only difference is that arrow blocks point to one expression, while brace blocks
can contain multiple expressions.

```
{ # has access to outer scope }

box { # NO access to outer scope }

int x = 10

int a = {
	int y = 20
	x + y
} # 30

int b = box {
	int y = 20
	x + y
} # !! ERROR x is undefined
```

### If/else if/else

```
int random_num = 8

if random_num is 5 => log ('is 5')
else if random_num is 2 => log ('is 2')
else => logs ('is', random_num)

if random_num > 0 {
	# do a lot of stuff
	# inside a brace block
	# "=>" and "{ }" are equivalent
}
```

### Loops

```
int x = 0
while x < 10 => x += 1

map [string, int] entries = [
	a: 10,
	b: 20,
	c: 30
]

# all three params
int val = for index, key, value in entries {
	if some_condition => break value
}

# just two params (key & value)
for k, v in entries {
	if some_condition => continue
	else {
		# do stuff
	}
}

# just one param
for val in entries => entries.val *= 10
# returns
```

## 5. Features

### Range

Creates an array with explicitly inclusive/exclusive start & end

```
0[..]5 # 1, 2, 3, 4
[0..5] # 0, 1, 2, 3, 4, 5
0[..5] # 1, 2, 3, 4, 5
[0..]5 # 0, 1, 2, 3, 4

int length = 10
[0..]length # easy way to avoid out-of-bounds errors
```
