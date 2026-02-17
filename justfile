#!/usr/bin/env just --justfile

# private is used so these two don't show up in the --list output
[private]
default: list
[private]
list:
    just --list

# Start a nix dev environment with `nix develop`
dev:
    nix develop . --command zsh -l

# Run cargo build with --release flag to build the binary
build:
	cargo build --release

# Run tests with cargo
test:
    cargo test

# Update coverage badge
coverage:
    ./helpers/coverage.sh

# Update list of Top-Level Domains in src/data/tlds.rs
update-tlds:
    ./helpers/update-tlds.sh

# Update the list of emojis
update-emoji:
    ./helpers/add-emojis.nu

# Update the rust data file with values from corpora JSON files
update-corpora-rust:
    ./helpers/corpora-data.sh

# Update the corpora submodule in data/corpora
update-corpora-git:
    git submodule update --init data/corpora

# Return the body for the match function in src/data.rs
get-corpora-match-body:
    grep "pub const" src/data/corpora.rs | awk -F':' '{ print $1 }' | awk -F ' ' '{ print "\""tolower(substr($3,6))"\" => corpora::"$3"," }'

# Return a list of all available dataset names, this is used for the function comment and the README!
get-corpora-available-dataset-names:
    grep "pub const" src/data/corpora.rs | awk -F':' '{ print $1 }' | awk -F ' ' '{ print tolower(substr($3,6)) }'

# Remove ./target directory which contains a lot of files
cleanup:
    rm -r target
