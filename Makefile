# Makefile that automates launching the infrastructure and running the tests.

launch:
	@echo "Building the Oracle"
	@cargo build --release --manifest-path=pcidss/Cargo.toml
	@echo "Launching the Oracle"
	@cd pcidss && export RUST_LOG=info && ./target/release/pcidss-oracle --dev
	@echo "Launching the payment processor"
	@yarn run --cwd=payment-processor dev-server
	@echo "Launching the merchant"
	@yarn run --cwd=interface start
