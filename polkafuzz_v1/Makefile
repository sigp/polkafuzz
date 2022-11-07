.PHONY: default help build fmt build-gfuzz clean  
WORKSPACE_DIR := $(realpath ../)
#
# Utils for polkafuzz_v1.
#
default: run

help:
	@echo 'Management commands for polkafuzz_v1'
	@echo
	@echo 'Usage:'
	@echo '    make build           Compile the project.'
	@echo '    make build-gfuzz     Compile the gossamer dependencies.'
	@echo '    make fmt             Run Rust and Go fmt.'
	@echo '    make update          Update submodules'
	@echo '    make clean           Clean the project artifacts.'	


# Compile the project
build:
	RUSTFLAGS="-Clink-arg=-Wl,--allow-multiple-definition" \
	cargo build --release --out-dir=. -Z unstable-options

# Build gfuzz
build-gfuzz:
	cp libs/gfuzz/gfuzz.go $(WORKSPACE_DIR)/clients/gossamer && \
	cd $(WORKSPACE_DIR)/clients/gossamer && \
	go build -o libgfuzz.a -buildmode=c-archive gfuzz.go && \
	mv gfuzz.go $(WORKSPACE_DIR)/polkafuzz_v1/libs/gfuzz/gfuzz.go && \
	mv libgfuzz.a $(WORKSPACE_DIR)/polkafuzz_v1/libs/gfuzz/libgfuzz.a && \
	mv libgfuzz.h $(WORKSPACE_DIR)/polkafuzz_v1/libs/gfuzz/libgfuzz.h

# Run polkafuzz_v1
run: build
	./polkafuzz_v1 list

# Run Rust and Go fmt to make code cleaner
fmt:
	cargo fmt --all
	cp libs/gfuzz/gfuzz.go $(WORKSPACE_DIR)/clients/gossamer && \
	cd $(WORKSPACE_DIR)/clients/gossamer && go fmt gfuzz.go && \
	mv gfuzz.go $(WORKSPACE_DIR)/polkafuzz_v1/libs/gfuzz/gfuzz.go

# Clean the project.
clean:
	cargo clean
	rm -rf polkafuzz_v1
	rm -rf Cargo.lock
	cd fuzz && cargo clean
	rm -rf fuzz/Cargo.lock
	rm -rf libpolkafuzz_v1.rlib
	

# Update the submodules
update:
	git submodule init && git submodule update
