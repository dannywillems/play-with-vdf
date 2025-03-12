build: ## Build the project
		cargo build --all-targets --all-features


release: ## Build the project in release mode
		cargo build --release --all-targets --all-features

test: ## Test the project with non-heavy tests and using native cargo test runner
		cargo test --all-features --release $(CARGO_EXTRA_ARGS) -- --nocapture --skip heavy $(BIN_EXTRA_ARGS)

test-doc: ## Test the project's docs comments
		cargo test --all-features --release --doc

check-format: ## Check the code formatting
		cargo +nightly fmt -- --check
		cargo sort --check

format: ## Format the code
		cargo +nightly fmt
		cargo sort

lint: ## Lint the code
		cargo clippy --all-features --all-targets --tests $(CARGO_EXTRA_ARGS) -- -W clippy::all -D warnings

help: ## Ask for help!
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

generate-doc: ## Generate the Rust documentation
		@echo "Generating the documentation."
		RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
		@echo "The documentation is available at: ./target/doc"
