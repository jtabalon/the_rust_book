# Packages and Crates

**Package**: one or more crates that provide a set of functionality.
- contains `Cargo.toml` that dsecribes how to build the crates

**Crate**: *binary* or *library* crate.
- Binary Crate: programs you compile to an executable that you can run (command line or server)
    - must have a function called `main` that defines what happens in the executable
- Library Crate: define functionality intended to be shared with ultiple projects
    - don't have a `main` function
    - don't compile to an executable

**Crate Root**: Source fine that the Rust compiler starts from and makes the root module of crate

**Package Rules**: 
- can contain at most one library crate
- can contain as many binary crates
- must contain *at least* one crate

**File Structure**
- `src/main.rs` -> binary crate
- `src/lib.rs` -> library crate
- it is possible to have both

NOTE: a package can have multiple binary crates by placing files in teh `src/bin` directory.
- each file will be a separate binary crate




