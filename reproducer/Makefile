.PHONY: default help build fmt build-glib clean  
WORKSPACE_DIR := $(realpath ../)
#
# Utils for reproducer.
#
default: run

help:
	@echo 'Management commands for reproducer'
	@echo
	@echo 'Usage:'
	@echo '    make build           Compile the project.'
	@echo '    make build-glib      Compile the gossamer dependencies.'
	@echo '    make fmt             Run Rust and Go fmt.'
	@echo '    make update          Update submodules'
	@echo '    make clean           Clean the project artifacts.'	


# Compile the project
build:
	RUSTFLAGS="-Clink-arg=-Wl,--allow-multiple-definition" \
	cargo build --out-dir=. -Z unstable-options

# Build glib
build-glib:
	cp libs/glib/glib.go $(WORKSPACE_DIR)/clients/gossamer && \
	cd $(WORKSPACE_DIR)/clients/gossamer && \
	go build -o libglib.a -buildmode=c-archive glib.go && \
	mv glib.go $(WORKSPACE_DIR)/reproducer/libs/glib/glib.go && \
	mv libglib.a $(WORKSPACE_DIR)/reproducer/libs/glib/libglib.a && \
	mv libglib.h $(WORKSPACE_DIR)/reproducer/libs/glib/libglib.h

# Run reproducer
run: build
	./reproducer list

# Run Rust fmt to make code cleaner
fmt:
	cargo fmt --all
	cp libs/glib/glib.go $(WORKSPACE_DIR)/clients/gossamer && \
	cd $(WORKSPACE_DIR)/clients/gossamer && go fmt glib.go && \
	mv glib.go $(WORKSPACE_DIR)/reproducer/libs/glib/glib.go

# Clean the project.
clean:
	cargo clean
	rm -rf reproducer
	rm -rf Cargo.lock

# Update the submodules
update:
	git submodule init && git submodule update
