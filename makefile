run:
	cargo watch -x 'run --release'
test:
	cargo watch -x "test --lib models -- --nocapture"
watch:
	cargo watch -x run