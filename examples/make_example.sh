mkdir packages
cd ..
cargo build --release
cp target/release/rostend examples/packages
cd examples/example_service
cargo build --release
cp target/release/example_service ../packages