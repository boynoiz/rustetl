# Local Testing

Test Rust + Polars code locally before deploying to Windmill.

## Why Test Locally?

- **Faster iteration**: Instant `cargo run` vs 20s Windmill compile
- **Better debugging**: Full Rust tooling, IDE support
- **Offline development**: Work without Windmill running
- **Learn Rust**: Understand compilation errors better

## Setup

```bash
cd local-testing
cargo build
```

## Running Examples

### Main binary
```bash
cargo run
```

### Specific example
```bash
cargo run --example test_parameterized
```

## Workflow

1. **Write code locally**
   ```bash
   # Edit src/main.rs or examples/
   cargo run
   ```

2. **Test and iterate**
   ```bash
   # Fast feedback loop
   cargo check  # Just check compilation
   cargo run    # Run it
   ```

3. **Copy to Windmill**
   - Adapt error handling (use `anyhow::Result`)
   - Add dependency comments at top
   - Test in Windmill

## Structure

```
local-testing/
├── Cargo.toml          # Dependencies
├── src/
│   ├── main.rs         # Main binary
│   └── parameterized.rs # Reusable modules
└── examples/
    └── test_*.rs       # Test examples
```

## Tips

- Use `?` for error handling locally
- Change to `.map_err(|e| e.to_string())?` for Windmill
- Test with `println!` debugging
- Use `cargo clippy` for linting
- Run `cargo fmt` before committing

## Common Commands

```bash
# Check code without compiling
cargo check

# Build in release mode (optimized)
cargo build --release

# Run with optimizations
cargo run --release

# Format code
cargo fmt

# Lint code
cargo clippy

# Run tests
cargo test
```
