# Rust Presentation

- Systems level language.
- Statically typed.
- No exceptions - errors as values.
- Memory safety is done with the borrow checker - more on that later.
- No nullptr, all `None` types have to be used in the `Option` enum (like std::optional in C++).
- No constructors, have to use functions to create type.  Like using static functions in C++.
- Destructors are "mimicked" with the `Drop` trait.

- Almost like C++, but the compiler enforces a very strict style guide.
- Mention `serde` and `tokio`.

- Rust cannot write ALL the possible safe apps, but all Rust apps should be safe.
- No function overloading in Rust.


## Borrow Checker

- More like a `std::move` checker :)

## Immutable by default

- Nice.

## Compiler Helps a lot

- Messages are super nice.

## Move semantics by default.

- In C++ we have to do `std::move` a lot if we want move semantics.  But in rust move semantics are the default.
- `Copy` vs `Clone`.

## Error handling

- Very nice error handling in rust.
- builtin ? operator.

## Standard tooling

- Cargo vs cmake/premake/etc...

## No Inheritance

- Has cool enums.

## Generic type inference and scope helpers

- Set variables outside of scope without type.
- Generics inference is quite nice.

## Pattern matching

Very nice pattern matching.
    - Nice.
    ```
    while let Some(Ok((mut manager, err))) = tasks.next().await {}
    ```
- Compiler helps you when you do not match all patterns.

## Iterators

-
