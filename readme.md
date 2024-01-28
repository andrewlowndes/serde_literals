# serde_literals
Add support for serialising and deserialing literals directly into enum unit variants.

## How to use
Add the crate to your Cargo.toml dependencies:
```toml
[dependencies]
serde_literals = "0.1.0"
```

Import and use one of the `LitBool`, `LitChar` and `LitInt` structs in the `#[serde(with = "..")` attribute. For example:
```rust
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Items {
    #[serde(with = "LitInt::<123>")]
    Num123,
    #[serde(with = "LitBool::<true>")]
    AlwaysTrue,
    #[serde(with = "LitChar::<'z'>")]
    SingleChar,
    OtherText(String),
}
```

The above items will be parsed into the `Items` struct in order. The equivilant Typescript typing would be `123 | true  | 'z' | string`.

If a string or float literal is required a custom struct must be created for each literal instance and then used.
```rust
lit_str!(LitAuto, "auto");
lit_str!(LitBlah, "blah");
lit_float!(Lit3_1, 3.1);

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Items {
    #[serde(with = "LitAuto")]
    Auto,
    #[serde(with = "LitBlah")]
    Blah,
    #[serde(with = "Lit3_1")]
    Num3Dot1,
}
```
The above will be the parsed from a type equivilant to `'auto' | 'blah' | 3.1`.

## How it works
The serde `with` attribute calls custom serialize and deserialize functions for the enum variant. The Lit... structs implement the custom serialize functions to parse against a specific literal provided.

The `serde(untagged)` parses each enum variant in the order specified - specific literal instances must be specified higher in the enum than the more generic encompassing types.

## Limitations
Since the serde attributes do not support supplying a function (must reference a struct/module by name only), a specific struct must be created for each literal needed. However, where const generics are supported (ints, char and bools currently) a generic struct can be used instead.

Once support for static strings in const generics has been added to Rust this will be much cleaner to use and not require the use of macros to create temporary structs.
