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

If you need more control over the **vast** array of [`rustfmt` configuration
options](https://rust-lang.github.io/rustfmt), you can use the second form:

```rust
let codegen = quote::quote!{
    async fn go() {
        let _ = Client::new().operation_id().send().await?;
    }
};
let config = Config {
    max_width: Some(45),
    ..Default::default()
};

let narrow_formatted = rustfmt_config(config, codegen).unwrap();
```

Note that in order to use unstable configuration options, you will need to have
a the nightly version of `rustfmt` installed.

---

Thanks to David Tolnay for so many tools including
[`cargo-expand`](https://github.com/dtolnay/cargo-expand) from which this
borrows.