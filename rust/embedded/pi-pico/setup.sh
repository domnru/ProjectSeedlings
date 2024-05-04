rustup update stable
rustup target add thumbv6m-none-eabi
cargo install flip-link

echo "Installing for single ''Pico''"
cargo install elf2uf2-rs --locked