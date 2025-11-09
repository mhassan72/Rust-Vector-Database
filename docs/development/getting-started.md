# Getting Started with Phenix-DB Development

This guide will help you set up your development environment and start contributing to Phenix-DB.

## Prerequisites

### Required Tools

- **Rust** (1.75 or later)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustup update
  ```

- **Git**
  ```bash
  # macOS
  brew install git
  
  # Ubuntu/Debian
  sudo apt-get install git
  ```

- **Build Tools**
  ```bash
  # macOS
  xcode-select --install
  
  # Ubuntu/Debian
  sudo apt-get install build-essential pkg-config libssl-dev
  ```

### Optional Tools

- **RocksDB** (for rocksdb-backend feature)
  ```bash
  # macOS
  brew install rocksdb
  
  # Ubuntu/Debian
  sudo apt-get install librocksdb-dev
  ```

- **CUDA Toolkit** (for GPU acceleration)
  - Download from [NVIDIA CUDA Toolkit](https://developer.nvidia.com/cuda-downloads)

- **OpenCL** (for GPU acceleration)
  ```bash
  # macOS - included in system
  
  # Ubuntu/Debian
  sudo apt-get install opencl-headers ocl-icd-opencl-dev
  ```

## Clone the Repository

```bash
git clone https://github.com/mhassan72/Phenix-DB.git
cd Phenix-DB
```

## Project Structure

```
Phenix-DB/
├── phenix-db/              # Main Rust codebase
│   ├── src/
│   │   ├── core/          # Core data structures
│   │   ├── mathematical/  # Mathematical operations
│   │   ├── memory/        # Memory substrate components
│   │   ├── storage/       # Storage tiers
│   │   ├── distributed/   # Distributed consciousness
│   │   ├── concurrency/   # Lock-free concurrency
│   │   ├── learning/      # Adaptive learning
│   │   ├── security/      # Encryption & security
│   │   ├── api/           # API layer
│   │   ├── observability/ # Metrics & tracing
│   │   └── lib.rs         # Library entry point
│   ├── Cargo.toml         # Dependencies & configuration
│   └── README.md
├── docs/                   # Documentation
│   ├── development/       # Developer guides
│   ├── architecture/      # System architecture
│   └── api/              # API documentation
└── .kiro/specs/          # Feature specifications
```

## Building the Project

### Basic Build

```bash
cd phenix-db

# Check for compilation errors
cargo check

# Build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release
```

### Build with Features

```bash
# Build with SIMD optimizations
cargo build --features simd

# Build with GPU acceleration
cargo build --features gpu

# Build with RocksDB backend
cargo build --features rocksdb-backend

# Build with all features
cargo build --features full
```

### Build Specific Binaries

```bash
# Build the server
cargo build --bin phenix-server

# Build the CLI
cargo build --bin phenix-cli
```

## Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_vector_creation

# Run tests for a specific module
cargo test core::vector

# Run tests with all features
cargo test --all-features
```

## Running the Server

```bash
# Run in development mode
cargo run --bin phenix-server

# Run in release mode
cargo run --release --bin phenix-server

# Run with custom configuration
PHENIX_CONFIG=./phenix-db.toml cargo run --bin phenix-server
```

## Using the CLI

```bash
# Show help
cargo run --bin phenix-cli -- --help

# Example commands (to be implemented)
cargo run --bin phenix-cli -- query --vector "[1.0, 2.0, 3.0]" --k 10
cargo run --bin phenix-cli -- insert --file entity.json
cargo run --bin phenix-cli -- stats
```

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Changes

Follow the [Code Organization](./code-organization.md) guidelines.

### 3. Run Tests

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

### 4. Commit Changes

```bash
git add .
git commit -m "feat: your feature description"
```

Follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `test:` - Test additions/changes
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `chore:` - Maintenance tasks

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

## Development Tools

### Cargo Commands

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check for outdated dependencies
cargo outdated

# Update dependencies
cargo update

# Generate documentation
cargo doc --open

# Run benchmarks
cargo bench
```

### IDE Setup

#### VS Code

Install extensions:
- rust-analyzer
- CodeLLDB (for debugging)
- Even Better TOML
- crates (dependency management)

#### IntelliJ IDEA / CLion

Install the Rust plugin from JetBrains Marketplace.

### Debugging

```bash
# Build with debug symbols
cargo build

# Run with debugger (VS Code)
# Use the Debug panel with rust-analyzer

# Print debug information
RUST_LOG=debug cargo run --bin phenix-server

# Backtrace on panic
RUST_BACKTRACE=1 cargo run --bin phenix-server
```

## Configuration

### Environment Variables

```bash
# Log level
export RUST_LOG=info,phenix_db=debug

# Configuration file
export PHENIX_CONFIG=./phenix-db.toml

# Storage paths
export PHENIX_HOT_TIER_PATH=/data/hot
export PHENIX_WARM_TIER_PATH=/data/warm
export PHENIX_COLD_TIER_PATH=/data/cold
```

### Configuration File

Copy the example configuration:

```bash
cp phenix-db.toml.example phenix-db.toml
```

Edit `phenix-db.toml` to customize settings. See [Configuration Guide](./configuration.md) for details.

## Common Issues

### Build Failures

**Issue**: RocksDB compilation fails on macOS
```bash
# Solution: Install RocksDB via Homebrew
brew install rocksdb

# Or disable the feature
cargo build --no-default-features
```

**Issue**: SIMD features not available
```bash
# Solution: Update Rust to latest version
rustup update stable
```

### Runtime Issues

**Issue**: Permission denied on storage paths
```bash
# Solution: Create directories with proper permissions
mkdir -p /data/{hot,warm,cold}
chmod 755 /data/{hot,warm,cold}
```

**Issue**: Port already in use
```bash
# Solution: Change port in configuration or kill existing process
lsof -ti:50051 | xargs kill -9
```

## Next Steps

- Read [Architecture Overview](../architecture/overview.md)
- Review [Code Organization](./code-organization.md)
- Check [Testing Guidelines](./testing.md)
- Explore [Mathematical Foundations](../architecture/mathematical-foundation.md)
- See [Implementation Tasks](../../.kiro/specs/phenix-vision-refactor/tasks.md)

## Getting Help

- **Documentation**: Check `docs/` directory
- **Issues**: [GitHub Issues](https://github.com/mhassan72/Phenix-DB/issues)
- **Discussions**: [GitHub Discussions](https://github.com/mhassan72/Phenix-DB/discussions)
- **Specs**: See `.kiro/specs/phenix-vision-refactor/` for detailed specifications

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for contribution guidelines.
