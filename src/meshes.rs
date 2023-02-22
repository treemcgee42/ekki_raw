use std::collections::HashMap;
use std::sync::Arc;

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
    map: HashMap<MeshBankKeys, Arc<Mesh>>,
}

impl MeshBank {
    pub fn initialize() -> Self {
        let map = HashMap::from([(
            MeshBankKeys::Cube,
            Arc::new(Mesh::new_from_slice(CUBE_VERTICES, CUBE_INDICES)),
        )]);

        Self { map }
    }

    pub fn get(&self, key: MeshBankKeys) -> Arc<Mesh> {
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
        color: [0.9, 0., 0.],
    },
    Vertex {
        // 1
        position: [0.5, 0.5, -0.5],
        color: [0.9, 0., 0.],
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
    // right face
    Vertex {
        // 4
        position: [0.5, 0.5, 0.5],
        color: [0.9, 0., 0.],
    },
    Vertex {
        // 5
        position: [0.5, -0.5, 0.5],
        color: [0.9, 0.9, 0.9],
    },
    // left face
    Vertex {
        // 6
        position: [-0.5, 0.5, 0.5],
        color: [0.9, 0., 0.],
    },
    Vertex {
        // 7
        position: [-0.5, -0.5, 0.5],
        color: [0.9, 0.9, 0.9],
    },
];

#[rustfmt::skip]
const CUBE_INDICES: &[u16] = &[
    // front face
    0, 1, 2,
    2, 1, 3, 
    // right face
    1, 4, 3,
    3, 4, 5,
    // left face
    0, 7, 6,
    0, 2, 7,
    // back face
    6, 7, 4,
    7, 5, 4,
    // top face
    1, 0, 6,
    1, 6, 4,
    // bottom face
    2, 3, 7,
    3, 5, 7,
];

// E==== CONSTANTS }}}1
