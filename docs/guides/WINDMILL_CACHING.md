# Windmill Rust Caching Behavior

## How Caching Works

Windmill caches compiled Rust binaries based on:
1. **Script content hash** - Changes to code trigger recompile
2. **Dependencies hash** - Changes to Cargo.toml trigger recompile
3. **Script version** - Each saved version has its own cache

## Cache Location

Cached binaries are stored in the Docker volume:
- Volume: `worker_dependency_cache`
- Path inside container: `/tmp/windmill/cache`

## Test Scenarios

### Scenario 1: Run Same Script Twice (No Changes)
**Expected:** Fast execution, uses cached binary

1. Run script first time → Slow (compiling)
2. Run script again → Fast (cached)
3. Check execution time difference

### Scenario 2: Change Code Slightly
**Expected:** Recompiles from scratch

1. Change a `println!` message
2. Save script
3. Run → Slow (recompiling)
4. Run again → Fast (new cache)

### Scenario 3: Add New Dependency
**Expected:** Full recompile with new dependencies

1. Add a new crate like `chrono = "0.4"`
2. Save script
3. Run → Very slow (downloading + compiling new deps)
4. Run again → Fast

### Scenario 4: Change Dependency Version
**Expected:** Recompiles with new version

1. Change `polars = "0.44"` to `polars = "0.43"`
2. Run → Slow (downloading different version)

### Scenario 5: Multiple Scripts
**Expected:** Each script has its own cache

1. Create Script A with Polars
2. Create Script B with Polars (same deps)
3. Both are cached separately

## Performance Expectations

### First Run (Cold Start)
- **Download dependencies**: 10-30 seconds (network dependent)
- **Compile Polars**: 60-120 seconds (CPU intensive)
- **Compile script**: 5-10 seconds
- **Total**: 2-3 minutes

### Cached Run (Warm Start)
- **Load binary from cache**: < 1 second
- **Execute**: Milliseconds to seconds (depends on data size)
- **Total**: 1-5 seconds

### Partial Cache Hit
If dependencies are cached but script changed:
- **Recompile script only**: 5-10 seconds
- **Total**: ~10 seconds

## Cache Persistence

The cache persists across:
- ✅ Container restarts (uses Docker volume)
- ✅ Windmill server restarts
- ✅ Multiple runs of same script

The cache is cleared when:
- ❌ Docker volume is deleted (`docker compose down -v`)
- ❌ Script content changes
- ❌ Dependencies change
- ❌ Manually clearing cache (if Windmill has a feature)

## Monitoring Cache Performance

Check execution time in Windmill UI:
- Look at "Duration" field in job results
- First run: ~120s for Polars scripts
- Cached runs: ~2-5s

## Test It Yourself

### Quick Test (5 minutes)

1. **First Run** (compilation):
   ```
   - Click "Run" in Windmill
   - Note the "Duration" time (expect ~2-3 min)
   - Check logs for "Compiling" messages
   ```

2. **Second Run** (cached):
   ```
   - Click "Run" again immediately
   - Note the "Duration" time (expect ~2-5 sec)
   - Much faster!
   ```

3. **Modify Code**:
   ```
   - Change println!("Original DataFrame:") to println!("Data:")
   - Click "Run"
   - Note: Slower again (recompiling)
   ```

4. **Run Modified Code Again**:
   ```
   - Click "Run"
   - Fast again (new cache)
   ```

### Check Cache Size

```bash
# Check cache volume size
docker volume inspect windmill_worker_dependency_cache

# See files in cache (from inside container)
docker exec -it windmill-windmill_worker-1 ls -lh /tmp/windmill/cache

# Check cache disk usage
docker exec -it windmill-windmill_worker-1 du -sh /tmp/windmill/cache
```

## Cache Optimization Tips

### 1. Lock Dependencies
Use exact versions to avoid cache invalidation:
```toml
[dependencies]
polars = "0.44.0"  # Exact version, not "0.44"
```

### 2. Shared Dependencies
If multiple scripts use Polars, they share dependency cache (but not binary cache).

### 3. Development vs Production
- **Development**: Accept slow first runs, benefit from fast iterations
- **Production**: Pre-warm cache by running scripts once after deployment

### 4. CI/CD Pipeline
In automated deployments:
```bash
# After deploying new scripts, warm the cache
curl -X POST "http://windmill/api/w/workspace/jobs/run/p/script_path"
```

## Comparison: Local vs Windmill

### Local Development (cargo)
```bash
cargo build --release
# First: ~2-3 minutes
# Incremental: ~5-10 seconds (smart recompilation)
```

### Windmill
```
First run: ~2-3 minutes (same as cargo)
Cached run: ~2-5 seconds (no compilation)
Code change: ~2-3 minutes (full recompile)
```

**Key Difference**: Windmill doesn't do incremental compilation (yet?).
It's all-or-nothing: either cached binary or full recompile.

## Real-World Impact

### Development Workflow
1. Write code locally with `cargo run` (fast iterations)
2. Test in Windmill when ready
3. First Windmill run is slow (expected)
4. Testing/debugging in Windmill is fast (cached)

### Production
- Scripts run frequently → Almost always cached
- Cache hit rate: ~99%+ for stable scripts
- Only recompiles on deployments

### Example Timeline
```
Day 1:
  Deploy script → 3 min (compile)
  Run 100 times → 5 sec each (cached)

Day 2:
  No changes → All runs cached (5 sec each)

Day 3:
  Update script → 3 min (recompile)
  Run 100 times → 5 sec each (new cache)
```

## Conclusion

Windmill's caching is **binary-level**:
- Fast when cached (seconds)
- Slow when not cached (minutes)
- No middle ground (no incremental compilation)

**Best Practice**:
- Develop locally with `cargo` for fast iterations
- Deploy to Windmill when code is stable
- Accept first-run compilation time
- Enjoy fast cached executions thereafter
