use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::bmesh_vertex::BMeshVertex;

/// There are many situations where purely storing vertices to implicitly define
/// edges is more compact than storing edges, e.g. storing two edges with a shared
/// vertex would require duplicating (a reference to) the shared vertex, where this
/// may not be necessary if we were storing just vertices and defining edges implicitly.
/// The benifit is storing associated data to the edge, which may be useful in
/// the future, and this is more readable.
pub struct BMeshEdge {
    id: u32,

    // The vertices the edge is defined between.
    v0: Rc<RefCell<BMeshVertex>>,
    v1: Rc<RefCell<BMeshVertex>>,
}

impl BMeshEdge {
    pub fn create(
        v0: Rc<RefCell<BMeshVertex>>,
        v1: Rc<RefCell<BMeshVertex>>,
        lookup_table: &mut BMeshEdgeLookupTable,
        rng: &mut impl rand::Rng,
    ) -> Rc<RefCell<Self>> {
        let to_return = Rc::new(RefCell::new(Self {
            id: rng.next_u32(),
            v0,
            v1,
        }));

        lookup_table.insert_edge(to_return);

        to_return
    }

    pub fn get_v0_id(&self) -> u32 {
        self.v0.as_ref().borrow().get_id()
    }

    pub fn get_v1_id(&self) -> u32 {
        self.v1.as_ref().borrow().get_id()
    }
}

/// Takes vertices as keys.
/// - Looking up with a single vertex will return all return all edges that share that
/// vertex.
/// - Looking up with two vertices will return the (unique) edge between them.
pub struct BMeshEdgeLookupTable {
    table: HashMap<u32, HashMap<u32, Rc<RefCell<BMeshEdge>>>>,
}

impl BMeshEdgeLookupTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::default(),
        }
    }

    pub fn insert_edge(&mut self, item: Rc<RefCell<BMeshEdge>>) {
        let key0 = item.as_ref().borrow().get_v0_id();
        let key1 = item.as_ref().borrow().get_v1_id();

        let v0_table = self.table.entry(key0).or_default();
        // TODO: warn if value existed already?
        v0_table.insert(key1, item.clone());

        let v1_table = self.table.entry(key1).or_default();
        v1_table.insert(key0, item.clone());
    }
}

pub struct BMeshEdgeLoop {
    defining_vertices: Vec<Rc<RefCell<BMeshVertex>>>,
}

impl BMeshEdgeLoop {
    pub fn new(defining_vertices: Vec<Rc<RefCell<BMeshVertex>>>) -> Self {
        Self { defining_vertices }
    }

    pub fn get_vertices(&self) -> Vec<Rc<RefCell<BMeshVertex>>> {
        self.defining_vertices.clone()
    }
}
