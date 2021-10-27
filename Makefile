###############################
# Common defaults/definitions #
###############################

comma := ,

# Checks two given strings for equality.
eq = $(if $(or $(1),$(2)),$(and $(findstring $(1),$(2)),\
                                $(findstring $(2),$(1))),1)




###########
# Aliases #
###########


all: fmt lint test docs


docs: cargo.doc


fmt: cargo.fmt


lint: cargo.lint


test: test.cargo




##################
# Cargo commands #
##################

# Generate crates documentation from Rust sources.
#
# Usage:
#	make cargo.doc [crate=<crate-name>] [private=(yes|no)]
#	               [open=(yes|no)] [clean=(no|yes)]

cargo.doc:
ifeq ($(clean),yes)
	@rm -rf target/doc/
endif
	cargo doc $(if $(call eq,$(crate),),--workspace,-p $(crate)) \
		--all-features \
		$(if $(call eq,$(private),no),,--document-private-items) \
		$(if $(call eq,$(open),no),,--open)


# Format Rust sources with rustfmt.
#
# Usage:
#	make cargo.fmt [check=(no|yes)]

cargo.fmt:
	cargo +nightly fmt --all $(if $(call eq,$(check),yes),-- --check,)


# Lint Rust sources with Clippy.
#
# Usage:
#	make cargo.lint

cargo.lint:
	cargo clippy --workspace --all-features -- -D warnings


cargo.test: test.cargo




####################
# Testing commands #
####################

# Run Rust tests of project.
#
# Usage:
#	make test.cargo [crate=<crate-name>]

test.cargo:
	cargo test $(if $(call eq,$(crate),),--workspace,-p $(crate)) --all-features




##################
# .PHONY section #
##################

.PHONY: all docs fmt lint test \
        cargo.doc cargo.fmt cargo.lint cargo.test \
        test.cargo
