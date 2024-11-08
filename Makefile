# Makefile

# Define the name of the executable
EXECUTABLE_NAME := aoc

# Define the target directory for the release build
TARGET_DIR := target/release

# Define the path to the executable
EXECUTABLE_PATH := $(TARGET_DIR)/$(EXECUTABLE_NAME)

# Default target: build and copy the executable
all: build

# Build the project in release mode and copy the resulting executable to the root directory
build:
	@cargo build --release
	@cp $(EXECUTABLE_PATH) .

.PHONY: all build