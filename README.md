# Windmill + Rust + Polars Learning Repository

A hands-on learning project for building fast, efficient ETL pipelines using Windmill, Rust, and Polars - replacing traditional Airflow + Python workflows.

## 🎯 Goals

- Learn Rust through practical data engineering tasks
- Master Polars for high-performance data transformations
- Build production-ready ETL pipelines with Windmill
- Replace memory-hungry Python/Pandas workflows (16GB → 1-2GB)
- Migrate from Airflow to Windmill for better performance and DX

## 📚 Project Structure

```
windmill/
├── docs/                          # Documentation
│   ├── LEARNING_PATH.md          # 10-week structured curriculum
│   ├── QUICK_START_RUST.md       # Get started in 5 minutes
│   └── guides/                    # Detailed guides
│       ├── WINDMILL_CACHING.md   # How caching works
│       ├── HOW_TO_USE_WINDMILL.md
│       └── DATABASE_WORKFLOW.md   # ETL workflow guide
│
├── windmill-scripts/              # Ready-to-use Windmill scripts
│   ├── 01-basics/                # Basic Polars operations
│   ├── 02-advanced/              # Advanced transformations
│   ├── 03-parameterized/         # Scripts with parameters
│   └── 04-database/              # Database ETL workflows
│
├── local-testing/                 # Local Rust development
│   ├── Cargo.toml
│   ├── src/                      # Test locally before Windmill
│   └── examples/
│
├── sample-data/                   # Test CSV files
│
├── docker-compose.yaml           # Windmill stack (running)
├── .env                          # Configuration
├── Caddyfile                     # Reverse proxy
├── .gitignore                    # Git ignore
├── LICENSE                       # MIT License
├── SETUP.md                      # Setup guide
├── CONTRIBUTING.md               # How to contribute
└── GIT_INIT.sh                   # Git initialization script
```

## 🚀 Quick Start

### 1. Start Windmill

```bash
cd ~/Projects/windmill
docker compose up -d
```

Wait for `windmill-full` image to download (~2-4GB).

### 2. Open Windmill UI

http://localhost:80

### 3. Create Your First Script

1. Click **"+ Script"** → Select **"Rust"**
2. Copy code from `windmill-scripts/01-basics/basic_polars.rs`
3. Click **"Run"**
4. First run: ~2-3 minutes (compiling)
5. Second run: ~20ms (cached!) 🚀

## 📖 Learning Path

### Week 1-2: Basics
- [x] Set up Windmill with Docker
- [x] Run first Polars script
- [x] Understand caching behavior
- [ ] Complete all basic examples

### Week 3-4: Intermediate
- [ ] Parameterized scripts
- [ ] CSV transformations
- [ ] Database connections

### Week 5-6: Advanced
- [ ] Complex ETL pipelines
- [ ] Data anonymization
- [ ] Multi-step workflows (Flows)

### Week 7+: Production
- [ ] Replace Airflow workflows
- [ ] Customer data migration pipeline
- [ ] Schedule and monitor jobs

See [docs/LEARNING_PATH.md](docs/LEARNING_PATH.md) for full curriculum.

## 🎓 Examples

### Basic: DataFrame Operations
```rust
let df = df! {
    "name" => &["Alice", "Bob"],
    "age" => &[25, 30],
}?;

let result = df
    .lazy()
    .filter(col("age").gt(lit(25)))
    .collect()?;
```

### Parameterized: Salary Calculator
- Input: CSV data + raise percentage
- Output: Transformed data + statistics
- See: `windmill-scripts/03-parameterized/`

### Database: ETL Workflow
- Generate fake customer data
- Anonymize sensitive fields (GDPR compliant)
- Create Windmill Flow (DAG)
- See: `windmill-scripts/04-database/`

## 📊 Performance Comparison

| Metric | Airflow + Python | Windmill + Rust |
|--------|-----------------|-----------------|
| Memory (100k records) | 16-17GB 😱 | 1-2GB ✅ |
| Startup Time | Seconds | 20ms ⚡ |
| Execution | Minutes | Seconds |
| Type Safety | Runtime errors | Compile-time |
| Setup Complexity | High | Low |

## 🛠️ Tech Stack

- **Windmill**: Workflow orchestration (Airflow alternative)
- **Rust**: Safe, fast systems programming
- **Polars**: High-performance DataFrame library (Pandas alternative)
- **PostgreSQL**: Database (included with Windmill)
- **Docker**: Easy deployment

## 📚 Resources

### Official Docs
- [Windmill Docs](https://www.windmill.dev/docs)
- [Polars Rust API](https://docs.rs/polars/latest/polars/)
- [Rust Book](https://doc.rust-lang.org/book/)

### This Repository
- [Learning Path](docs/LEARNING_PATH.md) - Structured 10-week plan
- [Quick Start](docs/QUICK_START_RUST.md) - Get coding in 5 minutes
- [Caching Guide](docs/guides/WINDMILL_CACHING.md) - Understand performance

## 🎯 Use Cases

### 1. Customer Data Migration
Replace slow Python scripts with fast Rust:
- Extract from old system
- Transform and validate
- Load to new system
- 10x faster, 90% less memory

### 2. Data Anonymization
GDPR-compliant data processing:
- Hash PII fields
- Bucket sensitive data
- Preserve analytics value
- See: `windmill-scripts/04-database/`

### 3. ETL Pipelines
Modern data pipelines:
- Replace Airflow DAGs with Windmill Flows
- Visual workflow editor
- Type-safe transformations
- Automatic caching

## 🚦 Getting Started Checklist

- [ ] Clone this repo
- [ ] Start Windmill: `docker compose up -d`
- [ ] Open UI: http://localhost:80
- [ ] Run first script from `windmill-scripts/01-basics/`
- [ ] Test locally in `local-testing/`
- [ ] Read [QUICK_START_RUST.md](docs/QUICK_START_RUST.md)
- [ ] Build your first workflow!

## 🤝 Contributing

This is a learning repository. Feel free to:
- Add more examples
- Improve documentation
- Share your workflows
- Report issues

## 📝 License

MIT License - Feel free to use for learning and production!

## 🎓 About

Created while learning Rust for production data engineering.

**Goal**: Replace Airflow + Python workflows with Windmill + Rust for better performance, lower costs, and happier developers.

**Background**: DevSecOps engineer working with Kubernetes, Docker, and data pipelines daily. Tired of Python's memory usage and Airflow's complexity.

---

**Start your journey**: [Quick Start Guide](docs/QUICK_START_RUST.md) 🚀
