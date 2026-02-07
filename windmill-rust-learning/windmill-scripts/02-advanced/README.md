# Advanced Polars Transformations

These scripts demonstrate advanced Polars features for complex data transformations.

## Scripts

### `advanced_transformations.rs`
- Multi-column calculations
- Group by multiple columns
- Window functions
- Percentage calculations
- Complex aggregations

**Use case**: Sales analytics, business intelligence

### `lazy_query_optimization.rs`
- Demonstrates lazy evaluation
- Query plan optimization
- Performance profiling
- Processing 1M+ rows efficiently

**Use case**: Understanding Polars' performance advantages

## Running in Windmill

1. Create new Rust script
2. Copy script content
3. Run and observe performance
4. Check logs for query plans

## Key Concepts

- **Window Functions**: Calculate over partitions
- **Lazy Evaluation**: Build query plan before execution
- **Query Optimization**: Polars automatically optimizes
- **Performance**: 10-100x faster than Pandas
