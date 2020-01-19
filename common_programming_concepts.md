## Variables and Mutability

- variables are by default immutable
- immutable = once a value is bound to a name, the value can't be changed
- `mut` in front of the variable name make it mutable

## Differences Between Variables and Constants

- constants aren't allowed to use `mut`
- constants are always immutable
- constants are declared using the `const` keyword instead of the `let` keyword
- constants need the type of the value be annotated
- constants can be declared in any scope, including the global scope

## Integer Literals

- type suffix can be appended to the value e.g. `57u8`
- `_` can be used as visual separator e.g. `1_000`
- hex = `0xff`
- octal = `0o77`
- binary = `0b1111_0000`
- byte = `b'A'`

## Integer Overflow

- in **debug** mode the program **panics** if a integer overflow occurs
- in **release** mode the value is 'wrapped around'
  - in case of a `u8`, 256 becomes 0, 257 becomes 1, and so on

## Character Type

- `char` type is four bytes in size and represents a Unicode Scalar Value
- can represent more that just ASCII

## Tuple Type

- general way of grouping together a number of values with a variety of types
- tuples have a fixed length

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

- to get the individual values out of a tuple, pattern matching can be used

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// destructure the tuple and break it into three parts
let (x, y, z) = tup;
```

- tuple elements can be directly accessed with a period (`.`)

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

let five_hundred = x.0;
let six_point_four = x.1;
let one = x.2;
```
