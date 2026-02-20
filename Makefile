SHELL := /bin/bash
RUSTFORGE_PATH ?= ../Rustforge

.PHONY: help
help:
	@echo "Starter Makefile"
	@echo "--------------"
	@echo "  make run-api"
	@echo "  make run-ws"
	@echo "  make run-worker"
	@echo "  make console CMD='route list'"
	@echo "  make route-list"
	@echo "  make migrate-pump"
	@echo "  make migrate-run"
	@echo "  make assets-publish ASSETS_ARGS='--from frontend/dist --clean'"
	@echo "  make framework-docs-build"
	@echo "  make check"
	@echo "  make gen"

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
