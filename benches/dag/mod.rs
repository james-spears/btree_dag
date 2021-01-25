use criterion::criterion_group;

mod bench_api;
pub use bench_api::*;

criterion_group!(
    dag_benches,
    clone_benchmark,
    vertices_benchmark,
    add_vertex_benchmark,
    add_edge_benchmark,
    get_vertex_value_benchmark,
    remove_vertex_benchmark,
    remove_edge_benchmark,
    adjacent_benchmark,
    connections_benchmark
);
