# for_let

[![Crates.io](https://img.shields.io/crates/v/for_let?style=for-the-badge)](https://crates.io/crates/for_let)
[![docs.rs](https://img.shields.io/docsrs/for_let?style=for-the-badge)](https://docs.rs/for_let/latest/for_let/)

That one syntax sugar library you (may have) wished you wrote yourself.

This library provides the `for_let!` macro, so you can write code like this:

```rust
for_let!(Some(Complex(Pattern(foo))) in iteratee {
    // do stuff
});
```

Which is just sugar for this:

```rust
for el in iteratee {
    match el {
        Some(Complex(Pattern(foo))) => {
            // do stuff
        }
        _ => {}
    }
}
```

Accepts all patterns that are legal in a match arm. How Pythonic!