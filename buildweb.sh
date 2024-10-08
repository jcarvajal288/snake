# make wasm_run
rustup target add wasm32-unknown-unknown  # should have been done previously
cargo install -f wasm-bindgen-cli  # or see https://crates.io/crates/wasm-server-runner
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./webbuild/out/ --target web ./target/wasm32-unknown-unknown/release/snake.wasm
cp -r assets ./webbuild/
# Copy the contents of index.html (see below), then run the following to replace project_name_here with your crate name from Cargo.toml
#PKG_NAME=$(grep '^name = ' Cargo.toml | awk -F '"' '{print $2}' | head -n 1)
#sed -i '' "s/\/out\/project_name_here.js/\/out\/${PKG_NAME}.js/g" webbuild/index.html
npx serve webbuild  # then navigate to http://localhost:3000