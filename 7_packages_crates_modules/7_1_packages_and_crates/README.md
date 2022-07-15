# Packages and Crates

**Package**: one or more crates that provide a set of functionality.
- contains `Cargo.toml` that dsecribes how to build the crates

**Crate**: *binary* or *library* crate.
- Binary Crate: programs you compile to an executable that you can run (command line or server)
    - must have a function called `main` that defines what happens in the executable
- Library Crate: define functionality intended to be shared with ultiple projects
    - don't have a `main` function
    - don't compile to an executable

