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
	cargo build --release -p dictionary --target wasm32-unknown-unknown
	cargo build --release -p dictionarya --target wasm32-unknown-unknown
	cargo build --release -p dictionaries --target wasm32-unknown-unknown
	cargo build --release -p testpackage --target wasm32-unknown-unknown
	cargo build --release -p storemultiple --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/listkeys.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/storetuple.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/accessright.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/accessright1.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/api_error.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/erc20_balance_of.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/listofkeys.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/multiplemaps.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/multipletuples.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/remove_key.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/return_resulttype.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/storelist.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/storemap.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/uref.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/dictionary.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/dictionarya.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/newdicc.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/newdica.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/testpackage.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/storemultiple.wasm 2>/dev/null | true

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