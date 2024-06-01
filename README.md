# sm-rs-data

This project aims to represent the upstream `sm-json-data` schema as idiomatic Rust. 

## Running Tests

I'm running tests using `cargo test`. Since this crate will function as a library, the tests will serve as documented proofs and examples for the API. In included a `launch.json` file for VSCode that should run the tests within the editor window. Or you can run it via command-line:

```cargo test --no-run --lib -- --show-output```