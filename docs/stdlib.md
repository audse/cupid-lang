# Standard library

## Builtin methods

Each builtin type has several associated method. Methods are documented in the
following format:

`[return_type] method_name (params...)`

Methods that **mutate** the initial value will include the param `mut self`,
whereas methods that **create a new value** will include the param `self`.

### Integer `int` 

1. [abs](#int-abs-self-source)
1. [clamp](#int-clamp-self-int-floor-int-ceil-source)
1. [negative](#bool-negative-self-source)
1. [positive](#bool-positive-self-source)
1. [sign](#int-sign-self-source)
1. [sq](#int-sq-self-source)

#### `[int] abs (self)` [source](./../stdlib/integer.cupid#L4)

Returns of the absolute value of `self`.

```
-100.abs() # 100

int mut num = -10
num = num.abs() # 10
```

#### `[int] clamp (self, int floor, int ceil)` [source](./../stdlib/integer.cupid#L10)

Returns `self` clamped to be no less than `floor` and no greater than `ceil`.

```
50.clamp(0, 10) # 10
-10.clamp(-5, 5) # -5
5.clamp(0, 10) # 5
```

#### `[bool] negative (self)` [source](./../stdlib/integer.cupid#L16)

Returns `true` if `self` is less than `0` and `false` otherwise.

```
10.negative() # false
-10.negative() # true
```

#### `[bool] positive (self)` [source](./../stdlib/integer.cupid#L20)

Returns `true` if `self` is greater than `0` and `false` otherwise.

```
10.positive() # true
-10.positive() # false
```

#### `[int] sign (self)` [source](./../stdlib/integer.cupid#L23)

Returns `-1` if `self` is a negative number, `1` if `self` is a positive number, and `0` if `self` is `0`.

```
-50.sign() # -1
1234.sign() # 1
0.sign() # 0
```

#### `[int] sq (self)`  [source](./../stdlib/integer.cupid#L30)

Returns the square of `self`.

```
12.sq() # 144
-3.sq() # -9
```

