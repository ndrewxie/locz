.PHONY: build clean

export CARGO_HOME = ./toolchain/cargo/

BINDGEN = ./toolchain/wasm_bindgen/wasm-bindgen
BUILDFLAGS = --package loczwasm --target wasm32-unknown-unknown --release
RUSTFLAGS = -C target-feature=+simd128,+bulk-memory,+nontrapping-fptoint,+sign-ext

BINPATH = ./target/wasm32-unknown-unknown/release/loczwasm.wasm
OUTDIR = ./client/locz
BINDFLAGS = --target web --keep-debug --no-typescript

build:
	rustup set profile minimal
	rustup default stable
	rustup target add wasm32-unknown-unknown
	rustup component add rustfmt
	rm -rf $(OUTDIR)/*
	cargo fmt --all
	cargo rustc $(BUILDFLAGS) -- $(RUSTFLAGS)
	$(BINDGEN) $(BINPATH) --out-dir $(OUTDIR) $(BINDFLAGS)

clean:
	cargo clean
	rm -rf ./toolchain/cargo/*
	rm -rf $(OUTDIR)/*