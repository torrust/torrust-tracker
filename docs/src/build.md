# Building UDPT
If you're reading this, you're probably interested in using UDPT - so first, thanks for your interest!

UDPT used to be harder to build, especially on Windows due to it's dependencies. Thanks to Rust, it's much simpler now.

## Required tools
- [Git](https://git-scm.com) - Version Control
- [Rust](https://www.rust-lang.org/) - Compiler toolchain & Package Manager (cargo)

### Getting the sources
```
git clone https://github.com/naim94a/udpt.git
```

If you prefer to just download the code, you can get the [latest codebase here](https://github.com/naim94a/udpt/archive/master.zip).

### Building
This step will download all required dependencies (from [crates.io](https://crates.io/)) and build them as well. 

Building should always be done with the latest Rust compiler.

```
cd udpt
cargo build --release
```

Once cargo is done building, `udpt` will be built at `target/release/udpt`.

### Running Tests
UDPT comes with unit tests, they can be run with the following command:
```
cargo test
```

If a build or test fails, please submit Issues to [UDPT's issue tracker](https://github.com/naim94a/udpt/issues).