# Setup Guide

Complete setup instructions for this learning repository.

## Prerequisites

- Docker & Docker Compose
- Git
- (Optional) Rust & Cargo for local testing

## Installation

### 1. Clone Repository

```bash
git clone <your-repo-url>
cd windmill-rust-learning
```

### 2. Start Windmill

```bash
docker compose up -d
```

**Note**: First time will download `windmill-full` image (~2-4GB).

### 3. Wait for Startup

```bash
# Check if services are ready
docker compose ps

# Follow logs
docker compose logs -f windmill_server
```

### 4. Access Windmill

Open browser: http://localhost:80

- Create an account
- Create a workspace

### 5. Test Setup

1. Create new Rust script
2. Copy from `windmill-scripts/01-basics/basic_polars.rs`
3. Run it
4. First run: ~2-3 minutes (compile)
5. Second run: ~20ms (cached)

If this works, you're ready! 🎉

## Optional: Local Rust Development

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Test Local Setup

```bash
cd local-testing
cargo run
```

## Troubleshooting

### Docker Issues

```bash
# Stop and remove everything
docker compose down -v

# Start fresh
docker compose up -d
```

### Port Already in Use

```bash
# Check what's using port 80
sudo lsof -i :80

# Change port in docker-compose.yml:
ports:
  - "8080:80"  # Use 8080 instead
```

### Rust Compilation Errors in Windmill

- Check you're using `windmill-full` image (has Rust compiler)
- Verify dependencies in cargo block
- Check logs: `docker compose logs windmill_worker`

### Database Connection Issues

From within Windmill scripts, use:
- Host: `db` (not `localhost`)
- User: `postgres`
- Password: `changeme`
- Database: `windmill`

## Next Steps

1. Read [Quick Start Guide](docs/QUICK_START_RUST.md)
2. Follow [Learning Path](docs/LEARNING_PATH.md)
3. Try examples in `windmill-scripts/`
4. Build something cool!

## Support

- Windmill Docs: https://www.windmill.dev/docs
- Polars Docs: https://docs.pola.rs/
- Rust Book: https://doc.rust-lang.org/book/
- Open an issue in this repo
