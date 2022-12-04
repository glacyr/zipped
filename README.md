# Zipped

Utility for recursively unzipping `tuple`s, `Option`s of `tuple`s and `Result`s
of `tuple`s.

## Install
```bash
cargo add zipped
```

## Usage
This crate is quiet straightforward.

### Unzipping `(((A, B), C), ...)`
If you have a left- or right-recursively zipped tuple, you can use
`UnzipInto::unzip_into` to turn it into a non-recursive tuple. This works for up
to 26 tuple elements.

```rust
use zipped::UnzipInto;

let (a, b, c) = ((1, 2), 3).unzip_into(); // left-recursive
let (a, b, c) = (1, (2, 3)).unzip_into(); // right-recursive
```

### Unzipping `Option<(((A, B), C), ...)>`
If you have an `Option` that contains a left- or right-recursively zipped tuple,
you can also use `UnzipInto::unzip_into` to turn it into an `Option` of a
non-recursive tuple. This also works for up to 26 tuple elements.

```rust
use zipped::UnzipInto;

let zipped = Some(1).zip(Some(2)).zip(Some(3));

match zipped.unzip_into() {
    Some((a, b, c)) => {}
    None => {}
}
```

While it's also possible to unzip `Option`s with right-recursively zipped
tuples, these don't occur naturally since `Option::zip` is left-recursive.

### Unzipping `Result<(((A, B), C), ...), E>`
If you have a `Result` that contains a left- or right-recursively zipped tuple,
you can also use `UnzipInto::unzip_into` to turn it into a `Result` of a
non-recursive tuple. This also works for up to 26 tuple elements.

```rust
use zipped::UnzipInto;

let zipped = Ok::<_, ()>(1)
    .and_then(|a| Ok((a, 2)))
    .and_then(|ab| Ok((ab, 3)));

match zipped.unzip_into() {
    Ok((a, b, c)) => {}
    Err(_) => {}
}
```

Again, while it's also possible to unzip `Result`s with right-recursively zipped
tuples, I found that these occur much less often.

## Limitations
- __Type inference.__ The compiler cannot automatically infer `T` in
  `UnzipInto<T>`. Eventually, you will need to specify the return value's arity.
- __Maximum arity.__ `UnzipFrom` is implemented for tuples of up to 26 elements.
- __Strict.__ It only works for completely zipped tuples where each tuple
  contains 2 elements and only the left (or the right) element can be another
  tuple, i.e. it does not work for `((A, B), C, D)`.

## License

Copyright 2022 Glacyr B.V.

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
