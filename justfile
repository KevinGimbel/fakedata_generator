#!/usr/bin/env just --justfile

# private is used so these two don't show up in the --list output
[private]
default: list
[private]
list:
    just --list

# Start a nix dev environment with `nix develop`
dev:
    nix develop --ignore-environment

# Run cargo build with --release flag to build the binary
build:
	cargo build --release

# Run tests with cargo
test:
    cargo test

# Remove compiled files
cleanup:
    rm -r target
