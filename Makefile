# Makefile

# Define the name of the executable
EXECUTABLE_NAME := aoc

# Define the target directory for the release build
TARGET_DIR := target/release

# Define the path to the executable
EXECUTABLE_PATH := $(TARGET_DIR)/$(EXECUTABLE_NAME)

# Default target: build and copy the executable
all: haskell python rust dev
build: haskell python rust

# Build the project in release mode and copy the resulting executable to the root directory
rust:
	@cargo build --release
	@cp $(EXECUTABLE_PATH) .

haskell:
	@stack build

python:
	@uv sync

dev:
	@uv sync
	@uv run pre-commit install

.PHONY: all rust haskell
