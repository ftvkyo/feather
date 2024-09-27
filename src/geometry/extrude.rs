use crate::geometry::P3;

use super::{AsPrimitives, Geometry2D, Geometry3D, FP};

impl Geometry2D {
    pub fn extrude_linear(&self, extent: FP) -> Geometry3D {
        let extent = extent / 2.0; // Go half down and half up (like `center = true` in OpenSCAD)

        /* Setup */

        let vs = self.vertices.len();
        let mut vertices = vec![];
        let mut triangles = vec![];

        /* Algorithm */

        // Add top vertices
        for v in &self.vertices {
            vertices.push(P3::new(v.x, v.y, extent));
        }
        // Add bottom vertices
        for v in &self.vertices {
            vertices.push(P3::new(v.x, v.y, - extent));
        }

        // Add top faces
        for t in &self.triangles {
            triangles.push(t.clone());
        }
        // Add bottom faces
        for [t0, t1, t2] in &self.triangles {
            // Note: opposite winding
            triangles.push([t0 + vs, t2 + vs, t1 + vs]);
        }

        // Add side faces
        for [e0, e1] in self.as_outer_edge_indices() {
            // Top triangle
            triangles.push([e0, e0 + vs, e1]);
            // Bottom triangle
            triangles.push([e1 + vs, e1, e0 + vs]);
        }

        Geometry3D {
            vertices,
            triangles,
        }
    }
}
