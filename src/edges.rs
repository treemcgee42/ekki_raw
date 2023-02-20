//! Defines the CPU-side preparation of edges for rendering, e.g. in
//! rendering wireframe outlines.
//!
//! We follow the approach in https://blog.mapbox.com/drawing-antialiased-lines-with-opengl-8766f34192d.

use crate::math::{point::Point3, vector::Vector2};

pub struct EdgeVertex {
    /// The 3D position of the vertex. The z-coordinate is only used for depth-testing.
    position: [f32; 3],
    /// A unit vector perpindicular to the point described by the first two coordinates of
    /// the vertex position. The sign will dictate which direction we push the vertex out;
    /// two `EdgeVertex`s describing the same position should have opposite sign normals.
    normal: [f32; 2],
}

impl EdgeVertex {
    pub fn new(position: Point3, normal: Vector2) -> Self {
        Self {
            position: position.into(),
            normal: normal.into(),
        }
    }
}

/// Convenience struct for specifying a collection of edges using an arbitrarily ordered collection
/// of positions and a sequence of indices. Rather than requiring the number of indices provided be
/// even, this struct makes it clear which indices should be considered together to define an edge.
pub struct EdgeIndex {
    point0_index: u16,
    point1_index: u16,
}

pub struct EdgeMesh {
    vertices: Vec<EdgeVertex>,
    indices: Vec<u16>,
}

impl EdgeMesh {
    // /// Create a new `EdgeMesh` given an arbitrarily ordered collection of vertices, where edges
    // /// are specified by `indices`.
    // pub fn new_from_vertices_and_indices(vertices: &Vec<Point3>, indices: &Vec<EdgeIndex>) -> Self {
    //     let mesh_vertices = Vec::<EdgeVertex>::with_capacity(vertices.len());
    //     let mesh_indices = Vec::<u16>::with_capacity(indices.len() * 6);

    //     for index in indices {
    //         let p0: Point3 = vertices.get(index.point0_index as usize).unwrap().clone();
    //         let p1 = vertices.get(index.point1_index.into()).unwrap();

    //         let edge_mesh = Self::new_from_two_points(&p0, &p1);
    //         mesh_vertices.append(&mut edge_mesh.vertices);
    //     }

    //     todo!()
    // }

    // /// Interpreting an edge between the two provided points, construct the mesh.
    // pub fn new_from_two_points(p0: &Point3, p1: &Point3) -> Self {
    //     let p0_to_p1 = (p1 - p0).xy();

    //     let normal0 = Vector2::get_perpindicular_cw_vector(p0_to_p1).normalize();
    //     let normal1 = -normal0;

    //     let v0 = EdgeVertex::new(p0, normal0);
    //     let v1 = EdgeVertex::new(p0, normal1);
    //     let v2 = EdgeVertex::new(p0, normal0);
    //     let v3 = EdgeVertex::new(p0, normal1);

    //     #[cfg_attr(rustfmt, rustfmt_skip)]
    //     Self {
    //         vertices: vec![v0, v1, v2, v3],
    //         indices: vec![
    //             0, 1, 2,
    //             1, 2, 3,
    //         ],
    //     }
    // }
}
