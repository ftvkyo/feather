pub mod primitives;
pub mod boolean;
pub mod extrude;

use cgmath::{AbsDiffEq, ElementWise, EuclideanSpace, Matrix2, Rad};
use primitives::*;


#[derive(Clone, Debug)]
pub struct IndexedTriangles<Point: Clone + std::fmt::Debug> {
    vertices: Vec<Point>,
    triangles: Vec<[usize; 3]>,
}

impl<Point: Clone + std::fmt::Debug> IndexedTriangles<Point> {
    pub fn new(vertices: Vec<Point>, triangles: Vec<[usize; 3]>) -> Self {
        Self {
            vertices,
            triangles,
        }
    }

    pub fn iter_vertices(&self) -> std::vec::IntoIter<Point> {
        let mut vs = vec![];

        for [t0, t1, t2] in &self.triangles {
            vs.push(self.vertices[*t0].clone());
            vs.push(self.vertices[*t1].clone());
            vs.push(self.vertices[*t2].clone());
        }

        vs.into_iter()
    }

    pub fn iter_triangles(&self) -> std::vec::IntoIter<Triangle<Point>> {
        let mut ts = vec![];

        for [t0, t1, t2] in &self.triangles {
            ts.push(Triangle::from_points([
                self.vertices[*t0].clone(),
                self.vertices[*t1].clone(),
                self.vertices[*t2].clone(),
            ]));
        }

        ts.into_iter()
    }

    /// Returns edges that are used by an odd number of triangles.
    /// For 2D objects, this is their outlines and holes.
    /// For 3D objects, this is what would make them non-manifold.
    pub fn outer_edge_indices(&self) -> Vec<[usize; 2]> {
        use std::collections::BTreeSet;

        let mut edges: BTreeSet<[usize; 2]> = BTreeSet::new();

        for [t0, t1, t2] in &self.triangles {
            for [a, b] in [[t0, t1], [t1, t2], [t2, t0]] {
                if edges.contains(&[*a, *b]) || edges.contains(&[*b, *a]) {
                    edges.remove(&[*a, *b]);
                    edges.remove(&[*b, *a]);
                } else {
                    edges.insert([*a, *b]);
                }
            }
        }

        // TODO: differentiate the outline and holes

        edges.into_iter().collect()
    }

    pub fn outer_edges(&self) -> Vec<[Point; 2]> {
        let indices = self.outer_edge_indices();
        let edges = indices.into_iter().map(|[a, b]|
            [self.vertices[a].clone(), self.vertices[b].clone()]
        ).collect();
        edges
    }

    pub fn concat(&self, other: &Self) -> Self {
        let vertices_count = self.vertices.len();
        let vertices: Vec<_> = self.vertices.iter().cloned().chain(other.vertices.iter().cloned()).collect();

        let triangles = self.triangles.iter().cloned().chain(other.triangles.iter().cloned().map(|t| {
            let [t0, t1, t2] = t;
            [t0 + vertices_count, t1 + vertices_count, t2 + vertices_count]
        })).collect();

        Self {
            vertices,
            triangles,
        }
    }

    pub fn translate(&self, vector: Point::Diff) -> Self
    where
        Point: EuclideanSpace
    {
        let vertices = self.vertices.iter().map(|vertex| {
            *vertex + vector
        }).collect();

        Self {
            vertices,
            triangles: self.triangles.clone(),
        }
    }

    pub fn scale(&self, vector: Point::Diff) -> Self
    where
        Point: EuclideanSpace + ElementWise
    {
        // FIXME: with negative scalings, triangle winding needs to be corrected

        let elwise = Point::from_vec(vector);
        let vertices = self.vertices.iter().map(|vertex| {
            vertex.mul_element_wise(elwise)
        }).collect();

        Self {
            vertices,
            triangles: self.triangles.clone(),
        }
    }
}

impl IndexedTriangles<P2> {

}

impl IndexedTriangles<P3> {
    pub fn as_manifold_edges(&self) -> Vec<[P3; 2]> {
        let mut edges = vec![];

        for [i0, i1, i2] in &self.triangles {
            if i0 < i1 {
                edges.push([self.vertices[*i0].clone(), self.vertices[*i1].clone()]);
            }
            if i1 < i2 {
                edges.push([self.vertices[*i1].clone(), self.vertices[*i2].clone()]);
            }
            if i2 < i0 {
                edges.push([self.vertices[*i2].clone(), self.vertices[*i0].clone()]);
            }
        }

        edges
    }
}

impl<Point> From<Triangles<Point>> for IndexedTriangles<Point>
where
    Point: AbsDiffEq<Point, Epsilon = FP> + Clone + std::fmt::Debug
{
    fn from(value: Triangles<Point>) -> Self {
        let mut vertices = vec![];
        let mut triangles = vec![];

        let find_vertex = |vs: &Vec<Point>, v: &Point| vs.iter()
            .position(|gv: &Point|
                cgmath::abs_diff_eq!(v, gv, epsilon = EPSILON)
            );

        for triangle in value.into_iter() {
            let mut global_indices = [0usize; 3];

            for (vi, v) in triangle.into_iter().enumerate() {
                if let Some(global_index) = find_vertex(&vertices, &v) {
                    global_indices[vi] = global_index;
                } else {
                    vertices.push(v);
                    global_indices[vi] = vertices.len() - 1;
                }
            }

            triangles.push(global_indices);
        }

        Self {
            vertices,
            triangles,
        }
    }
}


/* =========== *
 * 2D geometry *
 * =========== */

pub type Geometry2D = IndexedTriangles<P2>;

impl Geometry2D {
    pub fn rotate(&self, angle: Rad<FP>) -> Self {
        let rot = Matrix2::from_angle(angle);

        let vertices = self.vertices.iter().map(|vertex| {
            P2::from_vec(rot * vertex.to_vec())
        }).collect();

        Self {
            vertices,
            triangles: self.triangles.clone(),
        }
    }

    pub fn circle(sides: usize) -> Self {
        use cgmath::{Angle, Basis2, Rad, Rotation2, Rotation};

        assert!(sides >= 3, "Sides ({sides}) should be >= 3");

        let origin = P2::new(0.0, 0.0);
        let rot: Basis2<FP> = Rotation2::from_angle(Rad::full_turn() / sides as FP);

        let mut v = V2::new(1.0, 0.0);
        let mut outline = Outline2D(vec![]);

        for _ in 0..sides as u32 {
            outline.0.push(origin + v);
            v = rot.rotate_vector(v);
        }

        Self::try_from(outline).unwrap()
    }
}

impl TryFrom<Outline2D> for Geometry2D {
    type Error = anyhow::Error;

    fn try_from(value: Outline2D) -> Result<Self, Self::Error> {
        Ok(Self::from(Triangles::<P2>::try_from(value)?))
    }
}


/* =========== *
 * 3D geometry *
 * =========== */

pub type Geometry3D = IndexedTriangles<P3>;

impl Geometry3D {
    pub fn sphere(subdivisions: usize) -> Self {
        let subdivided = hexasphere::shapes::IcoSphere::new(subdivisions, |_| ());

        let vertices: Vec<P3> = subdivided
            .raw_points()
            .iter()
            .map(|p| p.to_array().map(|c| c as FP).into())
            .collect();

        let triangles: Vec<[usize; 3]> = subdivided
            .get_all_indices()
            .chunks(3)
            .map(|chunk| [chunk[0] as usize, chunk[1] as usize, chunk[2] as usize])
            .collect();

        Self {
            vertices,
            triangles,
        }
    }
}
