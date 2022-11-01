.PHONY: default help build fmt clean  
WORKSPACE_DIR := $(realpath ../polkafuzz_v2)
#
# Utils for polkafuzz_v2.
#
default: run

help:
	@echo 'Management commands for polkafuzz_v2'
	@echo
	@echo 'Usage:'
	@echo '    make build 										Compile the project.'
	@echo '    make fmt 										Run Rust fmt.'
	@echo '    make update										Update submodules'
	@echo '    make clean 										Clean the project artifacts.'	


# Compile the project
build:
	RUSTFLAGS="-Clink-arg=-Wl,--allow-multiple-definition" \
	cargo build --release --out-dir=. -Z unstable-options

# Run polkafuzz_v1
run: build
	./polkafuzz_v2 list

# Run Rust fmt to make code cleaner
fmt:
	cargo fmt --all

# Clean the project.
clean:
	cargo clean
	rm -rf polkafuzz_v2
	rm -rf Cargo.lock
	cd fuzzers/smoldot/fuzz && cargo clean
	rm -rf fuzzers/smoldot/fuzz/Cargo.lock
	cd fuzzers/substrate/fuzz && cargo clean
	rm -rf fuzzers/substrate/fuzz/Cargo.lock
	

# Update the submodules
update:
	git submodule init && git submodule update
