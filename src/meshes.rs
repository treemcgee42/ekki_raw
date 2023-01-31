use std::collections::HashMap;
use std::rc::Rc;

use crate::vertex::Vertex;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Mesh {
    fn new_from_slice(vertices: &[Vertex], indices: &[u16]) -> Self {
        Self {
            vertices: vertices.to_vec(),
            indices: indices.to_vec(),
        }
    }
}

// S==== MESH BANK {{{1

#[derive(PartialEq, Eq, Hash)]
pub enum MeshBankKeys {
    Cube,
}

/// This is a cache for default meshes.
pub struct MeshBank {
    // Could probably be an array, since we are only querying with enums
    map: HashMap<MeshBankKeys, Rc<Mesh>>,
}

impl MeshBank {
    pub fn initialize() -> Self {
        let map = HashMap::from([(
            MeshBankKeys::Cube,
            Rc::new(Mesh::new_from_slice(CUBE_VERTICES, CUBE_INDICES)),
        )]);

        Self { map }
    }

    pub fn get(&self, key: MeshBankKeys) -> Rc<Mesh> {
        if let Some(mesh) = self.map.get(&key) {
            return mesh.clone();
        }

        panic!()
    }
}

// E==== MESH BANK }}}1

// S==== CONSTANTS {{{1

const CUBE_VERTICES: &[Vertex] = &[
    // front face
    Vertex {
        // 0
        position: [-0.5, 0.5, -0.5],
        color: [0.9, 0.9, 0.9],
    },
    Vertex {
        // 1
        position: [0.5, 0.5, -0.5],
        color: [0.9, 0.9, 0.9],
    },
    Vertex {
        // 2
        position: [-0.5, -0.5, -0.5],
        color: [0.9, 0.9, 0.9],
    },
    Vertex {
        // 3
        position: [0.5, -0.5, -0.5],
        color: [0.9, 0.9, 0.9],
    },
];

#[rustfmt::skip]
const CUBE_INDICES: &[u16] = &[
    // front face
    0, 2, 1,
    2, 3, 1, 
];

// E==== CONSTANTS }}}1
