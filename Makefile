prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p listkeys --target wasm32-unknown-unknown
	cargo build --release -p storetuple --target wasm32-unknown-unknown
	cargo build --release -p accessright --target wasm32-unknown-unknown
	cargo build --release -p accessright1 --target wasm32-unknown-unknown
	cargo build --release -p api_error --target wasm32-unknown-unknown
	cargo build --release -p erc20_balance_of --target wasm32-unknown-unknown
	cargo build --release -p listofkeys --target wasm32-unknown-unknown
	cargo build --release -p multiplemaps --target wasm32-unknown-unknown
	cargo build --release -p multipletuples --target wasm32-unknown-unknown
	cargo build --release -p remove_key --target wasm32-unknown-unknown
	cargo build --release -p return_resulttype --target wasm32-unknown-unknown
	cargo build --release -p storelist --target wasm32-unknown-unknown
	cargo build --release -p storemap --target wasm32-unknown-unknown
	cargo build --release -p uref --target wasm32-unknown-unknown

clippy:
	cargo clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all
	
clean:
	cargo clean
	rm -rf target
	rm Cargo.lock