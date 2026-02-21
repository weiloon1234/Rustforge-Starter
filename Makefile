SHELL := /bin/bash
RUSTFORGE_PATH ?= ../Rustforge

ifneq (,$(wildcard ./.env))
	include ./.env
	export
endif

PUBLIC_PATH ?= public
FRAMEWORK_DOCS_PATH ?= /framework-documentation
FRAMEWORK_DOCS_ROUTE := $(patsubst /%,%,$(FRAMEWORK_DOCS_PATH))
FRAMEWORK_DOCS_DIR := $(PUBLIC_PATH)/$(FRAMEWORK_DOCS_ROUTE)

.PHONY: help
help:
	@echo "Starter Makefile"
	@echo "--------------"
	@echo "  make dev"
	@echo "  make run-api"
	@echo "  make run-ws"
	@echo "  make run-worker"
	@echo "  make console CMD='route list'"
	@echo "  make route-list"
	@echo "  make migrate-pump"
	@echo "  make migrate-run"
	@echo "  make server-install"
	@echo "  make server-update"
	@echo "  make assets-publish ASSETS_ARGS='--from frontend/dist --clean'"
	@echo "  make framework-docs-build"
	@echo "  make check"
	@echo "  make gen"

.PHONY: install-tools
install-tools:
	@command -v cargo-watch >/dev/null 2>&1 || cargo install cargo-watch

.PHONY: dev
dev:
	@command -v cargo-watch >/dev/null 2>&1 || (echo "cargo-watch not found. Run: make install-tools" && exit 1)
	RUN_WORKER=true cargo watch -x "run -p app --bin api-server"

.PHONY: run-api
run-api:
	./bin/api-server

.PHONY: run-ws
run-ws:
	./bin/websocket-server

.PHONY: run-worker
run-worker:
	./bin/worker

.PHONY: console
console:
	./bin/console $(CMD)

.PHONY: route-list
route-list:
	./bin/console route list

.PHONY: migrate-pump
migrate-pump:
	./bin/console migrate pump

.PHONY: migrate-run
migrate-run:
	./bin/console migrate run

.PHONY: server-install
server-install:
	sudo ./scripts/install-ubuntu.sh

.PHONY: server-update
server-update:
	./scripts/update.sh

.PHONY: assets-publish
assets-publish:
	./bin/console assets publish $(ASSETS_ARGS)

.PHONY: framework-docs-build
framework-docs-build:
	npm --prefix $(RUSTFORGE_PATH)/core-docs/frontend run build
	@mkdir -p "$(FRAMEWORK_DOCS_DIR)"
	@find "$(FRAMEWORK_DOCS_DIR)" -mindepth 1 -maxdepth 1 -exec rm -rf {} +
	cp -R "$(RUSTFORGE_PATH)/core-docs/frontend/dist/." "$(FRAMEWORK_DOCS_DIR)/"
	@echo "Published framework docs assets to $(FRAMEWORK_DOCS_DIR)"

.PHONY: check
check:
	cargo check --workspace

.PHONY: gen
gen:
	cargo build -p generated
