use crate::math::point::Point3;
use log::error;
use std::{cell::RefCell, rc::Rc};

use super::{
    bmesh_edge::{BMeshEdge, BMeshEdgeLookupTable, BMeshEdgeLoop},
    bmesh_face::{BMeshFace, TesselationStrategy},
    bmesh_vertex::BMeshVertex,
    BMeshTesselation,
};

pub struct BMesh {
    vertices: Vec<Rc<RefCell<BMeshVertex>>>,
    edges: Vec<Rc<RefCell<BMeshEdge>>>,
    faces: Vec<Rc<RefCell<BMeshFace>>>,

    edge_lookup_table: BMeshEdgeLookupTable,

    tesselation: BMeshTesselation,
}

impl BMesh {
    /// Assumes points go around the n-gon in a CCW fashion.
    fn from_ngon(rng: &mut impl rand::Rng, points: Vec<&Point3>) -> Self {
        let tesselation_strategy = match points.len() {
            3 => TesselationStrategy::Triangle,
            4 => TesselationStrategy::Quad,
            n => {
                error!("{}gon tesselation not implemented!", n);
                panic!();
            }
        };

        // Create vertices
        let vertices: Vec<Rc<RefCell<BMeshVertex>>> = points
            .iter()
            .map(|p| BMeshVertex::create_from_position((*p).clone(), rng))
            .collect();

        // Create edges
        let mut edge_lookup_table = BMeshEdgeLookupTable::new();
        let mut edges = Vec::with_capacity(vertices.len());

        for i in 0..vertices.len() {
            let v0 = i;
            let v1 = (i + 1) % vertices.len();
            let edge = BMeshEdge::create(
                vertices.get(v0).unwrap().clone(),
                vertices.get(v1).unwrap().clone(),
                &mut edge_lookup_table,
                rng,
            );

            edges.push(edge);
        }

        // Create face
        let face_edge_loop = BMeshEdgeLoop::new(vertices.clone());
        let face = BMeshFace::create(face_edge_loop, tesselation_strategy);

        let faces = vec![face];

        // Create tesselation
        let tesselation = BMeshFace::aggregate_tesselations(&faces);

        Self {
            vertices,
            edges,
            faces,
            edge_lookup_table,
            tesselation,
        }
    }

    //    #[rustfmt::skip]
    //    pub fn create_cube() -> Self {
    //        let p0 = Point3::new(-0.5, -0.5, -0.5); // front bottom left
    //        let p1 = Point3::new(-0.5, -0.5,  0.5); // back  bottom left
    //        let p2 = Point3::new( 0.5, -0.5,  0.5); // back  bottom right
    //        let p3 = Point3::new( 0.5, -0.5, -0.5); // fron  bottom right
    //
    //        let p4 = Point3::new(-0.5,  0.5,  0.5); // front top left
    //        let p5 = Point3::new( 0.5,  0.5,  0.5); // front top right
    //        let p6 = Point3::new( 0.5,  0.5, -0.5); // back  top right
    //        let p7 = Point3::new(-0.5,  0.5,  0.5); // back  top left
    //
    //        let p0_vertex = Rc::new(RefCell::new(BMeshVertex::new_from_position(p0)));
    //        let p1_vertex = Rc::new(RefCell::new(BMeshVertex::new_from_position(p1)));
    //        let p2_vertex = Rc::new(RefCell::new(BMeshVertex::new_from_position(p2)));
    //        let p3_vertex = Rc::new(RefCell::new(BMeshVertex::new_from_position(p3)));
    //
    //        let p4_vertex = Rc::new(RefCell::new(BMeshVertex::new_from_position(p4)));
    //        let p5_vertex = Rc::new(RefCell::new(BMeshVertex::new_from_position(p5)));
    //        let p6_vertex = Rc::new(RefCell::new(BMeshVertex::new_from_position(p6)));
    //        let p7_vertex = Rc::new(RefCell::new(BMeshVertex::new_from_position(p7)));
    //
    //        todo!()
    //    }
}
