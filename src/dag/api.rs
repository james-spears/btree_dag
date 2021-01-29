use alloc::collections::BTreeSet;

/// `Vertices` returns the set of the vertices which comprise the dag.
///
/// # Example
///
/// ```
/// use btree_dag::{BTreeDAG, AddVertex, Vertices};
/// let mut dag: BTreeDAG<String> = BTreeDAG::new();
///
/// assert_eq!(dag.vertices().len(), 0);
/// ```
pub trait Vertices<T>
where
    T: Ord,
{
    fn vertices(&self) -> BTreeSet<&T>;
}

/// `AddVertex` adds the vertex x, if it is not there.
///
/// # Example
///
/// ```
/// use btree_dag::{BTreeDAG, AddVertex, Vertices};
/// let mut dag: BTreeDAG<String> = BTreeDAG::new();
/// dag.add_vertex(String::from("origin"));
///
/// assert_eq!(dag.vertices().len(), 1);
/// assert!(dag.vertices().contains(&String::from("origin")));
/// ```
pub trait AddVertex<T>
where
    T: Ord,
{
    fn add_vertex(&mut self, x: T) -> Option<BTreeSet<T>>;
}

/// `AddEdge` add an edge from the vertex x to the vertex y, if it is not there.
///
/// # Example
///
/// ```
/// extern crate alloc;
/// use alloc::collections::btree_set::BTreeSet;
/// use btree_dag::{BTreeDAG, AddVertex, AddEdge, GetVertexValue};
/// use btree_dag::Error;
/// let mut dag: BTreeDAG<String> = BTreeDAG::new();
/// dag.add_vertex(String::from("origin"));
/// dag.add_vertex(String::from("destination"));
/// dag.add_edge(String::from("origin"), String::from("destination"));
///
/// let x_value: &BTreeSet<String> = dag.get_vertex_value(String::from("origin")).unwrap();
/// assert!(x_value.contains(&String::from("destination")));
///
/// assert!(dag.get_vertex_value(String::from("destination")).unwrap().is_empty());
/// ```
pub trait AddEdge<T> {
    type Error;
    fn add_edge(&mut self, x: T, y: T) -> Result<BTreeSet<T>, Self::Error>;
}

/// `GetVertexValue` returns the value associated with the vertex x.
///
/// # Example
///
/// ```
/// extern crate alloc;
/// use alloc::collections::btree_set::BTreeSet;
/// use btree_dag::{BTreeDAG, AddVertex, AddEdge, GetVertexValue};
/// let mut dag: BTreeDAG<String> = BTreeDAG::new();
/// dag.add_vertex(String::from("origin"));
/// dag.add_vertex(String::from("destination"));
/// dag.add_edge(String::from("origin"), String::from("destination"));
///
/// let vertex_value: &BTreeSet<String> = dag.get_vertex_value(String::from("origin")).unwrap();
/// assert!(vertex_value.contains(&String::from("destination")));
/// ```
pub trait GetVertexValue<T>
where
    T: Ord,
{
    fn get_vertex_value(&self, x: T) -> Option<&BTreeSet<T>>;
}

/// `RemoveEdge` removes the edge from the vertex x to the vertex y, if it is there.
///
/// # Example
///
/// ```
/// use btree_dag::{BTreeDAG, AddVertex, AddEdge, RemoveEdge, GetVertexValue};
/// let mut dag: BTreeDAG<String> = BTreeDAG::new();
/// dag.add_vertex(String::from("origin"));
/// dag.add_vertex(String::from("destination"));
/// dag.add_edge(String::from("origin"), String::from("destination"));
///
///
/// dag.remove_edge(String::from("origin"), String::from("destination"));
///
/// // Note: deletion of edges cascade i.e. the edge is also deleted from any incident
/// // vertices' adjacency lists.
/// assert_eq!(dag.get_vertex_value(String::from("origin")).unwrap().len(), 0);
/// assert_eq!(dag.get_vertex_value(String::from("destination")).unwrap().len(), 0);
/// ```
pub trait RemoveEdge<T> {
    type Error;
    fn remove_edge(&mut self, x: T, y: T) -> Result<BTreeSet<T>, Self::Error>;
}

/// `RemoveVertex` removes the vertex x, if it is there.
///
/// # Example
///
/// ```
/// extern crate alloc;
/// use alloc::collections::btree_set::BTreeSet;
/// use btree_dag::{BTreeDAG, AddVertex, AddEdge, RemoveVertex, GetVertexValue, Vertices};
/// let mut dag: BTreeDAG<String> = BTreeDAG::new();
/// dag.add_vertex(String::from("origin"));
/// dag.add_vertex(String::from("destination"));
/// dag.add_edge(String::from("origin"), String::from("destination"));
///
///
/// dag.remove_vertex(String::from("destination"));
/// assert_eq!(dag.vertices().len(), 1);
/// assert!(dag.vertices().contains(&String::from("origin")));
/// // Note: removing a vertex will also cascade delete any incident edges.
/// assert_eq!(dag.get_vertex_value(String::from("origin")).unwrap().len(), 0);
/// ```
pub trait RemoveVertex<T>
where
    T: Ord,
{
    type Error;
    fn remove_vertex(&mut self, x: T) -> Result<BTreeSet<T>, Self::Error>;
}

/// `Adjacent` tests whether there is an edge from the vertex x to the vertex y.
/// An error is thrown if either x, or y do not exist. By definition of adjacent there
/// must exist an edge e, with value (x, y) in order for vertices x, and y to be
/// considered adjacent.
///
/// # Example
///
/// ```
/// use btree_dag::{BTreeDAG, AddVertex, AddEdge, Adjacent};
/// use btree_dag::Error;
/// let mut dag: BTreeDAG<String> = BTreeDAG::new();
/// dag.add_vertex(String::from("origin"));
/// dag.add_vertex(String::from("destination"));
/// dag.add_edge(String::from("origin"), String::from("destination"));
///
/// assert!(dag.adjacent(String::from("origin"), String::from("destination")).unwrap());
/// // Note: the dag is directed, and the definition of adjacent
/// // can be phrased, if there exists a relationship from x to y. Therefore
/// // A and B adjacent implies B and A cannot be adjacent.
/// let err: Error = dag.add_edge(String::from("destination"), String::from("origin")).unwrap_err();
/// assert_eq!(err, Error::EdgeExists)
/// ```
pub trait Adjacent<T> {
    type Error;
    fn adjacent(&self, x: T, y: T) -> Result<bool, Self::Error>;
}

/// `Connections` lists all vertices y such that there is an edge from the vertex x to
/// the vertex y. An error is thrown if x does not exist.
///
/// # Example
///
/// ```
/// use btree_dag::{BTreeDAG, AddVertex, AddEdge, Connections};
/// let mut dag: BTreeDAG<String> = BTreeDAG::new();
/// dag.add_vertex(String::from("origin"));
/// dag.add_vertex(String::from("destination"));
/// dag.add_edge(String::from("origin"), String::from("destination"));
///
/// assert!(dag.connections(String::from("origin")).unwrap().contains(&String::from("destination")));
/// ```
pub trait Connections<T> {
    fn connections(&self, x: T) -> Option<&BTreeSet<T>>;
}

/// `Prune` remove vertex x and recursively remove all children
/// of x. Prune will error if x does not exists, and by `BTreeDAG`'s
/// contract, all recursive calls cannot error.
///
/// # Example
///
/// ```
/// use btree_dag::{BTreeDAG, AddVertex, AddEdge, Connections, Prune, Vertices, GetVertexValue};
/// use std::collections::BTreeSet;
/// let mut dag: BTreeDAG<String> = BTreeDAG::new();
/// dag.add_vertex(String::from("origin"));
/// dag.add_vertex(String::from("waypoint"));
/// dag.add_vertex(String::from("destination_A"));
/// dag.add_vertex(String::from("destination_B"));
///
/// dag.add_edge(String::from("origin"), String::from("waypoint"));
/// dag.add_edge(String::from("waypoint"), String::from("destination_A"));
/// dag.add_edge(String::from("waypoint"), String::from("destination_B"));
///
/// dag.prune(String::from("waypoint"));
///
/// assert_eq!(dag.vertices().len(), 1);
/// assert!(dag.vertices().contains(&String::from("origin")));
/// let remaining_children_of_origin: BTreeSet<String> = BTreeSet::new();
/// assert_eq!(dag.get_vertex_value(String::from("origin")).unwrap(), &remaining_children_of_origin);
/// ```
pub trait Prune<T> {
    type Error;
    fn prune(&mut self, x: T) -> Result<(), Self::Error>;
}