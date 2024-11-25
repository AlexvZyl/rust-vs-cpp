# Rust Presentation

- Systems level language.
- Statically typed.
- Memory safety is done with the borrow checker - more on that later.
- No nullptr, all `None` types have to be used in the `Option` enum (like std::optional in C++).
- No constructors, have to use functions to create type.  Like using static functions in C++.
- Destructors are "mimicked" with the `Drop` trait.

- Almost like C++, but the compiler enforces a very strict style guide.
- Mention `serde` and `tokio`.

- Rust cannot write ALL the possible safe apps, but all Rust apps should be safe.
- No function overloading in Rust.

- Immutable by default.
- In C++ we have to do `std::move` a lot if we want move semantics.  But in rust move semantics are the default.
- `Copy` vs `Clone`.

- No exceptions - errors as values.
- Very nice error handling in rust.
- builtin `?` operator.

- Cargo vs cmake/premake/etc...

- Set variables outside of scope without type.
- Generics inference is quite nice.

- Compiler helps you when you do not match all patterns.
## Pattern matching
- Very nice pattern matching.
    - Nice.
    ```
    while let Some(Ok((result))) = tasks.next().await {
        // do stuff...
    }
    ```
