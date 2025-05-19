.PHONY: data coverage

data:
	cd parse && cargo run --bin generate-data
	cargo test -- --nocapture

coverage:
	cargo tarpaulin --verbose --out html --exclude-files "parse/*"