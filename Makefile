#
# Test and build Social Context project
# This Makefile is primarily instructional; you can simply enter the Nix environment for
# holochain-rust development (supplied by holonix;) via `nix-shell` and run
# `make test` directly, or build a target directly.
#
SHELL		= bash

.PHONY: test test-unit test-dna

test: test-unit test-dna

test-unit:
		RUST_BACKTRACE=1 cargo test -- --nocapture

test-dna:
		cd zomes/tests &&  ( [ -d node_modules ] || npm install )  && npm run build-test
