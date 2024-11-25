# Rust Presentation

- Systems level language.
- Statically typed.
- Standardized tooling. Cargo vs cmake/premake/etc...
- Almost like C++, but the compiler enforces a very strict style guide.
- Immutable by default.

- Memory safety is done with the borrow checker - more on that later.
- In C++ we have to do `std::move` a lot if we want move semantics.  But in rust move semantics are the default.
- `Copy` vs `Clone`.

- No constructors, have to use functions to create type.  Like using static functions in C++.
- Destructors are "mimicked" with the `Drop` trait.

- Rust cannot write ALL the possible safe apps, but all Rust apps should be safe.
- No function overloading in Rust.
- Concurrency can still break, for example, deadlocks.

- No `nullptr`, all `None` types have to be used in the `Option` type (like std::optional in C++).
- No exceptions - errors as values.
- Very nice error handling in rust (builtin `?` operator).

- Very nice pattern matching.
- Compiler helps you when you do not match all patterns.

- Generics type inference is quite nice.
- Mention `serde` and `tokio`.
