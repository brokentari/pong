# A convenience script that builds the rust project and compiles it into 
# web assembly. It then moves the generated files into the Apache web server
# directory.

cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/rust_pong.wasm --out-dir ./pong_wasm --no-modules --no-typescript
sudo cp /home/saul/Documents/rust_pong/pong_wasm/* /var/www/pong/