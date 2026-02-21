SHELL := /bin/bash
RUSTFORGE_PATH ?= ../Rustforge

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

.PHONY: assets-publish
assets-publish:
	./bin/console assets publish $(ASSETS_ARGS)

.PHONY: framework-docs-build
framework-docs-build:
	npm --prefix $(RUSTFORGE_PATH)/core-docs/frontend run build

.PHONY: check
check:
	cargo check --workspace

.PHONY: gen
gen:
	cargo build -p generated
