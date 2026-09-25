SHELL := /bin/sh

DOPPLER ?= doppler
NPM ?= npm
DOPPLER_PROJECT ?=
DOPPLER_CONFIG ?=

DOPPLER_FLAGS :=
ifneq ($(strip $(DOPPLER_PROJECT)),)
DOPPLER_FLAGS += --project $(DOPPLER_PROJECT)
endif
ifneq ($(strip $(DOPPLER_CONFIG)),)
DOPPLER_FLAGS += --config $(DOPPLER_CONFIG)
endif

.PHONY: help setup dev dev-local require-doppler

help:
	@echo "Tro rebuild development commands"
	@echo "  make setup      Install locked Node and Python dependencies"
	@echo "  make dev        Start the full local stack with Doppler secrets"
	@echo "  make dev-local  Start the full fixture stack without Doppler"

setup:
	$(NPM) run setup

dev: require-doppler
	$(DOPPLER) run --forward-signals $(DOPPLER_FLAGS) -- $(NPM) run dev:full

dev-local:
	$(NPM) run dev:full

require-doppler:
	@command -v "$(DOPPLER)" >/dev/null 2>&1 || { \
		echo "Doppler CLI is required. Install it, then run 'doppler setup' in this repository."; \
		exit 1; \
	}
