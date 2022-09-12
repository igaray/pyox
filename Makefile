.PHONY: build run

default: help

help:
	@echo "pyox test"

build:
	@echo "Building..."
	maturin develop

run: build
	@echo "Running...\n"
	@python3 src/main.py

