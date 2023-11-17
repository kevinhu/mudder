# Mudder

Generate lexicographically-spaced strings between two strings from pre-defined alphabets.

This package is a rewrite of [Mudder.js](https://github.com/fasiha/mudderjs/tree/main) with a Rust core that is used to generate bindings to Python (via [Pyo3](https://github.com/PyO3/pyo3)) and JS/TS (via [wasm-pack](https://github.com/rustwasm/wasm-pack)).

# Quickstart

- Rust: `cargo add mudder`
- Python: `pip install mudderpy` or `poetry add mudderpy`
- JS/TS: `npm install mudderjs` or `yarn add mudderjs`

# API

The API is the same for all three languages. Note that there are a few differences in usage compared to the original:

- The `SymbolTable` constructor takes in a `str`/`string` instead of a `char[]`/`string[]`. Each member of the symbol table is assumed to be a character.
- When calling `mudder`, optional values for the `start` and `end` parameters use `None` or `undefined` instead of empty strings.
- No method overloads.

## Constructor

Create a new `SymbolTable` by passing in a string. The characters in the string will be used as the alphabet for the `SymbolTable`, with the first character being the "zero" character and, the second being the "one" character, and so on.

In Rust, the `new` method takes in a Vector of `char`s. The `from_str` method takes in a `&str`. In Python and JS/TS, the constructor takes in a `str`/`string`.

```rust
use mudder::SymbolTable;

let table = SymbolTable::from_str("abc");
```

```python
from mudderpy import SymbolTable

table = SymbolTable("abc")
```

```typescript
import { SymbolTable } from "mudderjs";

const table = new SymbolTable("abc");
```

## Default `SymbolTable`s

For convenience, there are a few default `SymbolTable`s that can be used.

- `SymbolTable::decimal`: `0123456789`
- `SymbolTable::alphabetic`: `abcdefghijklmnopqrstuvwxyz`
- `SymbolTable::base36`: `0123456789abcdefghijklmnopqrstuvwxyz`
- `SymbolTable::base62`: `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz`
- `SymbolTable::hex`: `0123456789abcdef`

## `SymbolTable` methods

A `SymbolTable` has the following methods:

- `SymbolTable::mudder(n: usize, start: Option<&str>, end: Option<&str>) -> Result<Vec<String>,  &'static str>`: Generate `n` strings between `start` and `end`. If `start` is `None`, the first string will be the first string in the `SymbolTable`. If `end` is `None`, the last string will be the last string in the `SymbolTable`, repeated `k+6` times where `k=len(start)`.

- `SymbolTable::mudder_one(start: Option<&str>, end: Option<&str>) -> Result<String,  &'static str>`: Convenience method for calling `mudder` with `n=1` and returning the first element of the resulting vector.

Note that for Python and JS, the return type is just a list of strings or a single string.

## Examples

```rust
use mudder::SymbolTable;

let table = SymbolTable::from_str("abc");
// let table = SymbolTable::new(vec!['a', 'b', 'c']);

let strings = table.mudder(5, None, None).unwrap();
assert_eq!(strings, vec!["ab", "ac", "b", "bc", "c"]);
```

```python
from mudderpy import SymbolTable

table = SymbolTable("abc")
strings = table.mudder(5)
assert strings == ['ab', 'ac', 'b', 'bc', 'c']
```

```typescript
import { SymbolTable } from "mudderjs";

const table = new SymbolTable("abc");
const strings = table.mudder(5);
assert(strings == ["ab", "ac", "b", "bc", "c"]);
```
