# Nama proyek
PROJECT_NAME := rust-rest-api

# Perintah Cargo
CARGO := cargo

# Port server
PORT := 3030

# Target default: build dan run
run: build
	@echo "Running $(PROJECT_NAME) on http://127.0.0.1:$(PORT)..."
	$(CARGO) run

# Build proyek
build:
	@echo "Building $(PROJECT_NAME)..."
	$(CARGO) build

# Build proyek dalam mode release
release:
	@echo "Building $(PROJECT_NAME) in release mode..."
	$(CARGO) build --release

# Jalankan binary hasil build release
run-release: release
	@echo "Running $(PROJECT_NAME) (release mode) on http://127.0.0.1:$(PORT)..."
	./target/release/$(PROJECT_NAME)

# Clean proyek
clean:
	@echo "Cleaning $(PROJECT_NAME)..."
	$(CARGO) clean

# Format kode
format:
	@echo "Formatting source code..."
	$(CARGO) fmt

# Lint kode
lint:
	@echo "Running clippy for linting..."
	$(CARGO) clippy -- -D warnings

# Jalankan unit test
test:
	@echo "Running tests for $(PROJECT_NAME)..."
	$(CARGO) test -- --nocapture

# Help command untuk daftar semua target
help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  run          Build and run the project"
	@echo "  build        Build the project"
	@echo "  release      Build the project in release mode"
	@echo "  run-release  Run the project in release mode"
	@echo "  clean        Clean build artifacts"
	@echo "  format       Format the source code using rustfmt"
	@echo "  lint         Run clippy to lint the source code"
	@echo "  test         Run the unit tests"
	@echo "  help         Show this help message"
