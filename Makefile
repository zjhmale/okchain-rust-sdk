.PHONY: install example doc test check

doc:
	cargo doc --no-deps

test:
	cargo test --all
	$(MAKE) example

check:
	cargo clippy
	cargo fmt --all -- --check

install:
	cargo install --path . -f

example:
	cargo run --example get_token

publish:
	cargo publish
