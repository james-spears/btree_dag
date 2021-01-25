use btree_dag::error::Error;
use btree_dag::*;
use criterion::{black_box, Criterion};

fn setup() -> Result<BTreeDag<String>, Error> {
    let mut dag: BTreeDag<String> = BTreeDag::new();
    dag.add_vertex(String::from("0"));
    dag.add_vertex(String::from("1"));
    dag.add_vertex(String::from("2"));
    dag.add_vertex(String::from("3"));
    dag.add_vertex(String::from("4"));
    dag.add_vertex(String::from("5"));
    dag.add_vertex(String::from("6"));
    dag.add_vertex(String::from("7"));
    dag.add_vertex(String::from("8"));
    dag.add_vertex(String::from("9"));

    dag.add_edge(String::from("0"), String::from("1"))?;
    dag.add_edge(String::from("0"), String::from("2"))?;
    dag.add_edge(String::from("0"), String::from("3"))?;
    dag.add_edge(String::from("0"), String::from("4"))?;
    dag.add_edge(String::from("0"), String::from("5"))?;
    dag.add_edge(String::from("0"), String::from("6"))?;
    dag.add_edge(String::from("0"), String::from("7"))?;
    dag.add_edge(String::from("0"), String::from("8"))?;
    dag.add_edge(String::from("0"), String::from("9"))?;

    dag.add_edge(String::from("1"), String::from("2"))?;
    dag.add_edge(String::from("1"), String::from("3"))?;
    dag.add_edge(String::from("1"), String::from("4"))?;
    dag.add_edge(String::from("1"), String::from("5"))?;
    dag.add_edge(String::from("1"), String::from("6"))?;
    dag.add_edge(String::from("1"), String::from("7"))?;
    dag.add_edge(String::from("1"), String::from("8"))?;
    dag.add_edge(String::from("1"), String::from("9"))?;

    dag.add_edge(String::from("2"), String::from("3"))?;
    dag.add_edge(String::from("2"), String::from("4"))?;
    dag.add_edge(String::from("2"), String::from("5"))?;
    dag.add_edge(String::from("2"), String::from("6"))?;
    dag.add_edge(String::from("2"), String::from("7"))?;
    dag.add_edge(String::from("2"), String::from("8"))?;
    dag.add_edge(String::from("2"), String::from("9"))?;

    dag.add_edge(String::from("3"), String::from("4"))?;
    dag.add_edge(String::from("3"), String::from("5"))?;
    dag.add_edge(String::from("3"), String::from("6"))?;
    dag.add_edge(String::from("3"), String::from("7"))?;
    dag.add_edge(String::from("3"), String::from("8"))?;
    dag.add_edge(String::from("3"), String::from("9"))?;

    dag.add_edge(String::from("4"), String::from("5"))?;
    dag.add_edge(String::from("4"), String::from("6"))?;
    dag.add_edge(String::from("4"), String::from("7"))?;
    dag.add_edge(String::from("4"), String::from("8"))?;
    dag.add_edge(String::from("4"), String::from("9"))?;

    dag.add_edge(String::from("5"), String::from("6"))?;
    dag.add_edge(String::from("5"), String::from("7"))?;
    dag.add_edge(String::from("5"), String::from("8"))?;
    dag.add_edge(String::from("5"), String::from("9"))?;

    dag.add_edge(String::from("6"), String::from("7"))?;
    dag.add_edge(String::from("6"), String::from("8"))?;
    dag.add_edge(String::from("6"), String::from("9"))?;

    dag.add_edge(String::from("7"), String::from("8"))?;
    dag.add_edge(String::from("7"), String::from("9"))?;

    Ok(dag)
}

pub fn clone_benchmark(c: &mut Criterion) {
    let dag = setup().unwrap();
    c.bench_function("dag::dag clone", |b| {
        b.iter(|| black_box(dag.clone()))
    });
}

pub fn vertices_benchmark(c: &mut Criterion) {
    let dag = setup().unwrap();
    c.bench_function("dag::api::Vertices", |b| {
        b.iter(|| black_box(dag.vertices()))
    });
}

pub fn add_vertex_benchmark(c: &mut Criterion) {
    let mut dag = setup().unwrap();
    c.bench_function("dag::api::AddVertex (vertex does not exist)", |b| {
        b.iter(|| black_box(dag.add_vertex(String::from("10"))))
    });

    c.bench_function("dag::api::AddVertex (vertex exists)", |b| {
        b.iter(|| black_box(dag.add_vertex(String::from("0"))))
    });
}

pub fn add_edge_benchmark(c: &mut Criterion) {
    let mut dag = setup().unwrap();
    c.bench_function("dag::api::AddEdge (edge does not exist)", |b| {
        b.iter(|| black_box(dag.add_edge(String::from("9"), String::from("0"))))
    });

    c.bench_function("dag::api::AddEdge (edge exists)", |b| {
        b.iter(|| black_box(dag.add_edge(String::from("0"), String::from("1"))))
    });
}

pub fn get_vertex_value_benchmark(c: &mut Criterion) {
    let dag = setup().unwrap();
    c.bench_function(
        "dag::api::GetVertexValue (vertex does not exist)",
        |b| b.iter(|| black_box(dag.get_vertex_value(String::from("10")))),
    );

    c.bench_function("dag::api::GetVertexValue (vertex exists)", |b| {
        b.iter(|| black_box(dag.get_vertex_value(String::from("0"))))
    });
}

pub fn remove_edge_benchmark(c: &mut Criterion) {
    let mut dag = setup().unwrap();
    c.bench_function("dag::api::RemoveEdge (edge does not exist)", |b| {
        b.iter(|| black_box(dag.remove_edge(String::from("1"), String::from("0"))))
    });

    c.bench_function("dag::api::RemoveEdge (edge exists)", |b| {
        b.iter(|| black_box(dag.remove_edge(String::from("9"), String::from("8"))))
    });
}

pub fn remove_vertex_benchmark(c: &mut Criterion) {
    let mut dag = setup().unwrap();
    c.bench_function("dag::api::RemoveVertex (vertex does not exist)", |b| {
        b.iter(|| black_box(dag.remove_vertex(String::from("10"))))
    });

    c.bench_function("dag::api::RemoveVertex (vertex exists)", |b| {
        b.iter(|| black_box(dag.remove_vertex(String::from("0"))))
    });
}

pub fn adjacent_benchmark(c: &mut Criterion) {
    let dag = setup().unwrap();
    c.bench_function("dag::api::Adjacent (vertices are not adjacent)", |b| {
        b.iter(|| black_box(dag.adjacent(String::from("9"), String::from("0"))))
    });

    c.bench_function("dag::api::Adjacent (vertices are adjacent)", |b| {
        b.iter(|| black_box(dag.adjacent(String::from("0"), String::from("1"))))
    });

    c.bench_function("dag::api::Adjacent (vertex does not exist)", |b| {
        b.iter(|| black_box(dag.adjacent(String::from("10"), String::from("1"))))
    });
}

pub fn connections_benchmark(c: &mut Criterion) {
    let dag = setup().unwrap();
    c.bench_function("dag::api::Connections (vertex does not exist)", |b| {
        b.iter(|| black_box(dag.connections(String::from("10"))))
    });

    c.bench_function("dag::api::Connections (vertex exists)", |b| {
        b.iter(|| black_box(dag.connections(String::from("0"))))
    });

    c.bench_function("dag::api::Connections (vertex exists)", |b| {
        b.iter(|| black_box(dag.connections(String::from("8"))))
    });

    c.bench_function("dag::api::Connections (vertex exists)", |b| {
        b.iter(|| black_box(dag.connections(String::from("9"))))
    });
}
