.PHONY: data coverage

data:
	mkdir -p data
	cd data && test -d sprites || git clone https://github.com/PokeAPI/sprites.git
	cd data/sprites && git pull
	cd parse && cargo run --release --bin generate-data
	cargo test --release -- --nocapture

coverage:
	cargo tarpaulin --verbose --out html --exclude-files "parse/*"