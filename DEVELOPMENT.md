# bitsnap

Operational Intelligence Platform

## Development

Install [asdf](https://asdf-vm.com/) first.

```bash
# assuming you've cloned the repo to ~/src/bitsnap

asdf plugin add rust
asdf plugin add golang
asdf plugin add python
asdf install
asdf current # should list all the correct versions from .tools-versions file

rustup default nightly
rustup component add rust-analyzer
rustup component add clippy-preview
rustup component add rustfmt
rustup component add miri

cargo install cargo-nextest --locked
cargo install cargo-mutants --locked
cargo install cargo-sonar --locked
cargo install cargo-llvm-cov --locked
cargo install licensure --locked

# double check that rust sources are available and change `Cargo.toml` path, if necessary
rust_nightly_date="nightly-2025-05-13" # rustc 1.88.0
rustup install "${rust_nightly_date}" 

cargo build

# extend PATH for cargo, if none was previously available
echo "export PATH=\"\$PATH:\$HOME/.cargo/bin\"" >> ~/.zshrc # or .bashrc

# load rust env for the current rust version specified in .tools-versions file
echo ". \"\$HOME/.asdf/installs/rust/$(cat .tool-versions| grep rust | awk '{print $2}')/env\"" >> ~/.zshrc # or .bashrc

# Testing
cargo nextest r --all  
cargo mutants --workspace -j 32 # for mutation testing with 32 concurrent jobs
cargo miri nextest run --all -j 32    # for miri mem-leak testing
cargo miri test
cargo miri run

# Coverage reporting
cargo llvm-cov --workspace --html --open

# Linting & Formatting
licensure -i bitsnap/platform/**/*.rs
licensure -i bitsnap/integration/**/*.rs
rustfmt --edition 2024 bitsnap/platform/**/*.rs
rustfmt --edition 2024 bitsnap/integration/**/*.rs

# commit changes
cargo clippy --fix  
```

## License
