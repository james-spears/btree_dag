# Binary Tree Dag (btree_dag)

![CodeBuild]
[![Version badge]][crates.io]
[![Docs badge]][docs.rs]

[CodeBuild]: https://codebuild.us-east-1.amazonaws.com/badges?uuid=eyJlbmNyeXB0ZWREYXRhIjoiQ3dscGZZSUxvTm0wU0R2SVVGVnJ0dnh1bENNazNVaW42VncwcTZVOVZEeVBBZzRvaUNDZEV4Rm1xQ3kycVM3cmR5MGN0dC9iQkZMbXNkeG52Uk9yMG1RPSIsIml2UGFyYW1ldGVyU3BlYyI6InVRby9LeitwNTVkN0JTUWIiLCJtYXRlcmlhbFNldFNlcmlhbCI6MX0%3D&branch=main
[Version badge]: https://img.shields.io/crates/v/btree_dag
[crates.io]: https://crates.io/crates/btree_dag
[Docs badge]: https://img.shields.io/badge/docs.rs-rustdoc-blue
[docs.rs]: https://docs.rs/btree_dag/

This library is a minimal implementation of a directed acyclic graph
(abstract data structure) by way of a single binary tree map
(`BTreeMap`). This implementation is often referred to as
an adjacency list.

The primary goals of this implementation are to be 
minimal and idiomatic to the Rust language. The `alloc`
crate is the only dependency when compiled with default
features and is not optional. As one might assume, `alloc`
is required for reason the implementation relies on `BTreeMap`
(and the `BTreeSet` wrapper).

## Example
```rust
use crate::BTreeDAG;

fn main() {
    let mut dag: BTreeDAG<String> = BTreeDAG::new();
    // Add nodes.
    dag.add_vertex(String::from("Tarzan"));
    dag.add_vertex(String::from("Jane"));
    // Add a relationship.
    dag.add_edge(String::from("Tarzan"), String::from("Jane"));
    
    // Assert relationship now exists.
    assert!(dag.adjacdent(String::from("Tarzan"), String::from("Jane")));
    
    // Adding a bidirectional edge is not allowed.
    assert!(dag.add_edge(String::from("Jane"), String::from("Tarzan"))?.is_err());
}
```

## Usage

Add the following to your `Cargo.toml` file:
```toml
[dependencies]
btree_dag = "0.1.0"
```

## API

Please see the [API](src/dag/api.rs) for a full list of
available methods.

## License

This work is dually licensed under MIT OR Apache-2.0.