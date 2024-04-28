# Session 1: Notes

Things to further learn based on this session:
1. Pain points - borrow checker, lifetimes (explicit), boxing, the power of traits, handling generics.
2. Workspaces - crates, libraries.

## General Notes

Cargo init
Cargo.toml - heart
How modules are divided are developer dependent, however it is typically best practice to be singular responsibility.

Test run with the following syntax - tells the compiler to ignore this during application build:
`#[cfg(test)]`

Linting a formatting:
1. Cargo format
2. Cargo clippy - goes beyond format allows for custom rules to apply idiomatic rust.

Crate is a fancy name for library/ package.
Crates.io presents all publicly available crates.

Most common memory used in rust is the stack.