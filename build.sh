export RUSTFLAGS="-C linker=/usr/bin/arm-linux-gnueabi-gcc"
#cargo build --target=arm-unknown-linux-musleabihf
cargo build --release --target=arm-unknown-linux-gnueabi
