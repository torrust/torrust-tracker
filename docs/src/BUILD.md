# Building Torrust Tracker

## Required tools
- [Git](https://git-scm.com) - Version Control
- [Rust](https://www.rust-lang.org/) - Compiler toolchain & Package Manager (cargo)

### Getting the sources
```bash
git clone https://github.com/torrust/torrust-tracker.git
```

If you prefer to just download the code, you can get the [latest codebase here](https://github.com/torrust/torrust-tracker/archive/master.zip).

### Building
This step will download all required dependencies (from [crates.io](https://crates.io/)) and build them as well. 

Building should always be done with the latest Rust compiler.

```bash
cd torrust-tracker
cargo build --release
```

Once cargo is done building, `torrust-tracker` will be built at `target/release/torrust-tracker`.
