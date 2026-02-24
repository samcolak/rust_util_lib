use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use util_lib::btree::Btree;

fn dataset_paths(depth: usize, width: usize) -> Vec<String> {
    let mut out = Vec::with_capacity(depth * width);
    for d in 0..depth {
        for w in 0..width {
            out.push(format!("/{d}/{w}/leaf"));
        }
    }
    out
}

fn build_tree(paths: &[String]) -> Btree<String> {
    let mut tree = Btree::new("/");
    for (index, path) in paths.iter().enumerate() {
        let value = format!("v{index}");
        let _ = tree.insert(path, value);
    }
    tree
}

fn bench_insert(c: &mut Criterion) {
    let paths = dataset_paths(50, 20);

    c.bench_function("btree_insert_1000", |b| {
        b.iter_batched(
            || Btree::new("/"),
            |mut tree| {
                for (index, path) in paths.iter().enumerate() {
                    let _ = tree.insert(black_box(path), black_box(format!("v{index}")));
                }
                black_box(tree);
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_fetch(c: &mut Criterion) {
    let paths = dataset_paths(50, 20);
    let tree = build_tree(&paths);
    let target = &paths[777];

    c.bench_function("btree_fetch_clone", |b| {
        b.iter(|| {
            let values = tree.fetch(black_box(target));
            black_box(values)
        })
    });

    c.bench_function("btree_fetch_ref", |b| {
        b.iter(|| {
            let values = tree.fetch_ref(black_box(target));
            black_box(values)
        })
    });
}

fn bench_items(c: &mut Criterion) {
    let paths = dataset_paths(100, 20);
    let tree = build_tree(&paths);

    c.bench_function("btree_items_clone_all", |b| {
        b.iter(|| {
            let items = tree.items();
            black_box(items)
        })
    });
}

fn bench_node_lookup(c: &mut Criterion) {
    let paths = dataset_paths(50, 20);
    let tree = build_tree(&paths);
    let target = "/10/3";

    c.bench_function("btree_node_for_clone", |b| {
        b.iter(|| {
            let node = tree.node_for(black_box(target));
            black_box(node)
        })
    });

    c.bench_function("btree_node_for_ref", |b| {
        b.iter(|| {
            let node = tree.node_for_ref(black_box(target));
            black_box(node)
        })
    });
}

criterion_group!(
    btree_benches,
    bench_insert,
    bench_fetch,
    bench_items,
    bench_node_lookup
);
criterion_main!(btree_benches);
