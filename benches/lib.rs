use criterion::criterion_main;

mod dag;
use dag::*;

criterion_main!(dag_benches,);
