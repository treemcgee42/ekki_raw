use std::{cell::RefCell, rc::Rc};

use crate::math::point::Point3;

use self::{bmesh_edge::BMeshEdge, bmesh_face::BMeshFace, bmesh_vertex::BMeshVertex};

mod bmesh;
mod bmesh_edge;
mod bmesh_face;
mod bmesh_vertex;

pub struct BMeshTesselation {
    vertices: Vec<Rc<RefCell<BMeshVertex>>>,
    indices: Vec<usize>,
    // Passed to the vertex
}

/// Representation of a circular linked list, for use in `BMesh`.
pub struct BMeshCycle<T> {
    data: Vec<T>,
    current_index: usize,
}

impl<T> BMeshCycle<T> {
    fn increment_current_index(&self) -> usize {
        (self.current_index + 1) % self.data.len()
    }

    /// Returns a reference to the item specified in the internal `data` vector
    /// at index `current_index`. The internal structure may be opaque to the caller,
    /// so this should be though of as returning the item you are currently on in a
    /// linked list.
    ///
    /// Panics if internal `current_index` is out of bounds.
    pub fn get(&self) -> &T {
        self.data.get(self.current_index).unwrap()
    }

    /// Returns (a reference to) the next item, looping back around to the beginning
    /// of the internal `data` vector if we are at the end, like a circular linked list.
    /// This function does not "advance" the structure, e.g. the value of `get()` will
    /// be the same before and after calling this function (see `advance()`).
    pub fn next(&self) -> &T {
        self.data.get(self.increment_current_index()).unwrap()
    }

    pub fn advance(mut self) -> Self {
        self.current_index = self.increment_current_index();
        self
    }
}

impl<T> From<Vec<T>> for BMeshCycle<T> {
    fn from(v: Vec<T>) -> Self {
        Self {
            data: v,
            current_index: 0,
        }
    }
}

pub struct BMeshMetadata {
    /// Used during tesselation to set up an index buffer.
    index: usize,
}
