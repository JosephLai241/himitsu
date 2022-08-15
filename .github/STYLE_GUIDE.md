# Style Guide

I only have a few rules:

1. **You have to use [`rustfmt`][rustfmt] with its default settings to format your code.** I recommend setting up your editor so that `rustfmt` will automatically format your code every time you save a file.
2. **Everything that may be documented requires a docstring describing what it is, including the module itself.** Examples of documentable items include structs, functions, and constants.
3. **All fields in `Cargo.toml` needs to be sorted in alphabetical order**. This includes package and bin metadata, as well as all dependencies. This makes it much easier for everyone to find things.

[rustfmt]: https://github.com/rust-lang/rustfmt
