# rustfmt-wrapper

Rust makes it easy to generate more Rust code--for macros, builders, whatever.
It's nice to format that code to be a little easier on the eyes. While
`rustfmt` does a pretty good job, it does a pretty good job as a command and
isn't usable as a library. This is a probably-too-simple wrapper library.

It's pretty simple to use and mixes well with `quote!`:

```rust
let codegen = quote::quote!{ struct Foo { bar: String } };
let formatted: String = rustfmt_wrapper::rustfmt(codegen).unwrap();
```

Thanks to David Tolnay for so many tools including `cargo-expand` from which
this borrows.