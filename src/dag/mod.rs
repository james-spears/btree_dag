mod api;
mod test;

use alloc::collections::{BTreeMap, BTreeSet};
use core::default::Default;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use api::*;
use crate::error::Error;

/// `BTreeDag` is an implementation of a directed acyclic graph (abstract data structure)
/// which utilizes `BTreeMap` for the vertex adjacency list.
#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BTreeDag<T>
where
    T: Ord,
{
    vertices: BTreeMap<T, BTreeSet<T>>,
}

impl<T> BTreeDag<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        let vertices: BTreeMap<T, BTreeSet<T>> = BTreeMap::new();
        BTreeDag { vertices }
    }

    fn cyclic_relationship_exists(&self, x: &T, y: &T) -> Result<(), Error> {
        if let Some(adj_y) = self.vertices.get(y) {
            // If y has adjacent vertices, then have we need to
            // check if x exists in these adjacent vertices;
            if !adj_y.contains(x) {
                // if it does not, then recurse. Making sure x
                // is not adjacent to any of y's adjacent vertices.
                for adj in adj_y {
                    self.cyclic_relationship_exists(x, adj)?;
                }
                // If no error has been thrown by this line, then
                // we must not have found x in any of the adjacency lists.
                return Ok(());
            }
            return Err(Error::EdgeExistsError);
        }
        // If y has no adjacent vertices, then we can be sure there
        // no circular relationship.
        Ok(())
    }
}

impl<T> Default for BTreeDag<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Vertices<T> for BTreeDag<T>
where
    T: Ord,
{
    fn vertices(&self) -> BTreeSet<&T> {
        self.vertices.keys().collect()
    }
}

impl<T> AddVertex<T> for BTreeDag<T>
where
    T: Ord,
{
    fn add_vertex(&mut self, x: T) -> Option<BTreeSet<T>> {
        self.vertices.insert(x, BTreeSet::new())
    }
}

/// When you add an edge, you should make sure that the x, and y vertices exist.
impl<T> AddEdge<T> for BTreeDag<T>
where
    T: Ord + Clone,
{
    type Error = Error;
    fn add_edge(&mut self, x: T, y: T) -> Result<(), Self::Error> {
        if self.vertices.get(&y).is_some() {
            if let Some(adj_x) = self.vertices.get(&x) {
                self.cyclic_relationship_exists(&x, &y)?;
                // Add y to x's adjacency list.
                let mut adj_x: BTreeSet<T> = adj_x.clone();
                adj_x.insert(y.clone());

                self.vertices.insert(x, adj_x);
                return Ok(());
            }
        }
        Err(Error::VertexDoesNotExist)
    }
}

impl<T> GetVertexValue<T> for BTreeDag<T>
where
    T: Ord,
{
    fn get_vertex_value(&self, v: T) -> Option<&BTreeSet<T>> {
        self.vertices.get(&v)
    }
}

/// When an edge is removed, you should find the incident vertex and ensure the edge
/// is removed from the vertex's adjacency list.
impl<T> RemoveEdge<T> for BTreeDag<T>
where
    T: Ord + Clone,
{
    type Error = Error;
    fn remove_edge(&mut self, x: T, y: T) -> Result<(), Self::Error> {
        if self.vertices.get(&y).is_some() {
            if let Some(adj_x) = self.vertices.get(&x) {
                // Remove y from x's adjacency list.
                let mut adj_x = adj_x.clone();
                adj_x.remove(&y);

                // Update vertices.
                self.vertices.insert(x, adj_x);
                return Ok(());
            }
        }
        Err(Error::VertexDoesNotExist)
    }
}

/// When you remove a vertex, you should ensure there are no dangling edges.
impl<T> RemoveVertex<T> for BTreeDag<T>
where
    T: Ord + Clone,
{
    type Error = Error;
    fn remove_vertex(&mut self, x: T) -> Result<(), Self::Error> {
        self.vertices
            .clone()
            .into_iter()
            .filter(|v| -> bool { v.1.contains(&x) })
            .try_for_each(|v| { self.remove_edge(v.0.clone(), x.clone()) })?;
        // At this point, no other vertices should point to x,
        // and so x can be removed.
        self.vertices.remove(&x);
        Ok(())
    }
}

impl<T> Adjacent<T> for BTreeDag<T>
where
    T: Ord,
{
    type Error = Error;
    fn adjacent(&self, x: T, y: T) -> Result<bool, Self::Error> {
        if self.vertices.get(&y).is_some() {
            if let Some(adj_x) = self.vertices.get(&x) {
                if adj_x.contains(&y) {
                    return Ok(true);
                }
                return Ok(false);
            }
        }
        Err(Error::VertexDoesNotExist)
    }
}

impl<T> Connections<T> for BTreeDag<T>
where
    T: Ord,
{
    fn connections(&self, x: T) -> Option<&BTreeSet<T>> {
        self.vertices.get(&x)
    }
}
