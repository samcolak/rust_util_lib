# Utility Library

Custom library for standard operations that i use alot...
Enjoy and feel free to use at your descretion

## Benchmarks

This project includes Criterion benchmarks for `Btree` in [benches/btree_bench.rs](benches/btree_bench.rs).

### Run benchmarks

```bash
cargo bench --bench btree_bench
```

### Compare two runs (baseline vs current)

Create a baseline from your current branch/state:

```bash
cargo bench --bench btree_bench -- --save-baseline main
```

After code changes, compare against that baseline:

```bash
cargo bench --bench btree_bench -- --baseline main
```

Or use the helper script:

```bash
./scripts/bench_compare.sh
```

Optional arguments:

```bash
./scripts/bench_compare.sh <bench_name> <baseline_name>
```

### How to interpret results

- Lower mean/median time is better.
- In comparison output, negative change means improvement; positive change means regression.
- Prefer changes that are larger than normal noise and are consistent across runs.
- Focus first on hot-path benchmarks:
	- `btree_fetch_ref` vs `btree_fetch_clone`
	- `btree_node_for_ref` vs `btree_node_for_clone`
	- `btree_insert_1000`
	- `btree_items_clone_all`

