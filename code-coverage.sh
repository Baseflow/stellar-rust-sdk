CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test --manifest-path=./stellar_rust_sdk/Cargo.toml
grcov . --binary-path ./stellar_rust_sdk/target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o ./stellar_rust_sdk/target/coverage/html
rm ./stellar_rust_sdk/*.profraw
# open the coverage report in the browser
xdg-open ./stellar_rust_sdk/target/coverage/html/index.html