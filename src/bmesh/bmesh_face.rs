use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{bmesh_edge::BMeshEdgeLoop, BMeshTesselation};

pub enum TesselationStrategy {
    Triangle,
    Quad,
}

pub struct BMeshFace {
    defining_edges: BMeshEdgeLoop,
    tesselation: BMeshTesselation,
}

impl BMeshFace {
    pub fn create(
        defining_edges: BMeshEdgeLoop,
        tesselation_strategy: TesselationStrategy,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            defining_edges,
            tesselation: Self::tesselate_edge_loop(&defining_edges, tesselation_strategy),
        }))
    }

    fn tesselate_edge_loop(
        edge_loop: &BMeshEdgeLoop,
        tesselation_strategy: TesselationStrategy,
    ) -> BMeshTesselation {
        let vertices = edge_loop.get_vertices();
        let indices = match tesselation_strategy {
            TesselationStrategy::Triangle => vec![0, 1, 2],
            TesselationStrategy::Quad => vec![0, 1, 2, 2, 3, 0],
        };

        BMeshTesselation { vertices, indices }
    }

    pub fn aggregate_tesselations(faces: &Vec<Rc<RefCell<BMeshFace>>>) -> BMeshTesselation {
        // for every face
        //     for every vertex in face.tesselation
        //        if vertex in index_lookup:
        //            indices.push(i)
        //        else:
        //            let i = vertices.len()
        //            index_lookup.insert(v, i)
        //            indices.push(i)
        //            vertices.push(v)

        let mut vertices = Vec::new();
        let mut indices: Vec<usize> = Vec::new();
        let mut index_lookup: HashMap<u32, usize> = HashMap::new();

        for face in faces {
            for vertex in face.as_ref().borrow().tesselation.vertices {
                let vertex_id = vertex.as_ref().borrow().get_id();

                if let Some(i) = index_lookup.get(&vertex_id) {
                    indices.push(i.clone());
                    continue;
                }

                let i = vertices.len();
                index_lookup.insert(vertex_id, i);
                indices.push(i);
                vertices.push(vertex.clone());
            }
        }

        BMeshTesselation { vertices, indices }
    }
}
