PYTHON ?= python3

LITERATE_SOURCE := src/lib.rs
LITERATE_SCRIPT := tools/rust_literate.py
LITERATE_OUTPUT := docs/lib.md

.DEFAULT_GOAL := literate

.PHONY: literate check-literate

literate: $(LITERATE_OUTPUT)

$(LITERATE_OUTPUT): $(LITERATE_SOURCE) $(LITERATE_SCRIPT)
	$(PYTHON) $(LITERATE_SCRIPT) $(LITERATE_SOURCE) $(LITERATE_OUTPUT)

check-literate:
	@tmp=$$(mktemp); \
	$(PYTHON) $(LITERATE_SCRIPT) $(LITERATE_SOURCE) $$tmp; \
	if ! cmp -s $$tmp $(LITERATE_OUTPUT); then \
		rm -f $$tmp; \
		echo "$(LITERATE_OUTPUT) is stale; run 'make -C code/minimal-dd-trial literate'"; \
		exit 1; \
	fi; \
	rm -f $$tmp
