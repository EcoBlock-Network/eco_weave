use criterion::{criterion_group, criterion_main, Criterion};
use eco_weave::{Tangle, Transaction};

pub fn benchmark_weighted_random_walk(c: &mut Criterion) {
    let mut tangle = Tangle::new();

    // Générer le graphe de benchmark
    for i in 1..=1000 {
        let node_id = format!("tx{}", i);
        tangle.add_node(&node_id);

        if i > 1 {
            let prev_id = format!("tx{}", i - 1);
            tangle.connect_nodes(&node_id, &prev_id);
        }

        let tx = Transaction::new(&node_id, "Payload").unwrap();
        tangle.add_transaction(tx);
    }

    // Ajouter le benchmark
    c.bench_function("weighted_random_walk_large_graph", |b| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        b.iter(|| runtime.block_on(tangle.weighted_random_walk("tx1")));
    });
}

criterion_group!(benches, benchmark_weighted_random_walk);
criterion_main!(benches);
