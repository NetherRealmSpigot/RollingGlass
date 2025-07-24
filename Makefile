all: lint test build

.PHONY: build
build:
	cargo build --release

.PHONY: test
test:
	cargo tarpaulin --force-clean --release --run-types AllTargets --out lcov --out stdout

.PHONY: lint
lint:
	cargo clippy --all-features -- --deny warnings

.PHONY: lint-fix
lint-fix:
	cargo clippy --all-features --fix

.PHONY: clean
clean:
	cargo clean
