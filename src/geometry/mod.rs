pub mod primitives;
pub mod boolean;
pub mod extrude;

use cgmath::{ElementWise, EuclideanSpace, Matrix2, Rad};
use primitives::*;


pub trait Geometry<V, R> {
    fn concat(&self, other: &Self) -> Self;
    fn translate(&self, v: V) -> Self;
    fn scale(&self, v: V) -> Self;
    fn rotate(&self, v: R) -> Self;
}

impl<T: AsPrimitives<P2> + Clone> Geometry<V2, Rad<FP>> for T {
    // TODO: Make transformations happen using matrices
    // (maybe this would allow generalising rotation between 2D and 3D shapes?)

    fn concat(&self, other: &Self) -> Self {
        let mut vs = self.get_vertices().clone();
        let vsn = vs.len();

        vs.extend(other.get_vertices().clone());

        let mut ts = self.get_triangles().clone();

        for [t0, t1, t2] in other.get_triangles() {
            ts.push([t0 + vsn, t1 + vsn, t2 + vsn]);
        }

        Self::from_primitives(vs, ts)
    }

    fn translate(&self, v: V2) -> Self {
        let p = P2::from_vec(v);
        let mut vs = self.get_vertices().clone();
        for vertex in &mut vs {
            vertex.add_assign_element_wise(p);
        }
        Self::from_primitives(vs, self.get_triangles().clone())
    }

    fn scale(&self, v: V2) -> Self {
        // FIXME: with negative scalings, triangle winding needs to be corrected

        let p = P2::from_vec(v);
        let mut vs = self.get_vertices().clone();
        for vertex in &mut vs {
            vertex.mul_assign_element_wise(p);
        }
        Self::from_primitives(vs, self.get_triangles().clone())
    }

    fn rotate(&self, v: Rad<FP>) -> Self {
        let rot = Matrix2::from_angle(v);

        let mut vs = self.get_vertices().clone();
        for vertex in &mut vs {
            *vertex = P2::from_vec(rot * vertex.to_vec());
        }
        Self::from_primitives(vs, self.get_triangles().clone())
    }
}


/* =========== *
 * 2D geometry *
 * =========== */

 #[derive(Clone, Debug)]
pub struct Geometry2D {
    vertices: Vec<P2>,
    triangles: Vec<[usize; 3]>,
}

impl AsPrimitives<P2> for Geometry2D {
    fn from_primitives(vertices: Vec<P2>, triangles: Vec<[usize; 3]>) -> Self {
        Self {
            vertices,
            triangles,
        }
    }

    fn get_vertices(&self) -> &Vec<P2> {
        &self.vertices
    }

    fn get_triangles(&self) -> &Vec<[usize; 3]> {
        &self.triangles
    }
}

impl Geometry2D {
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

impl From<Triangles<P2>> for Geometry2D {
    fn from(value: Triangles<P2>) -> Self {
        let mut vertices = vec![];
        let mut triangles = vec![];

        let find_vertex = |vs: &Vec<P2>, v: &P2| vs.iter()
            .position(|gv: &P2|
                cgmath::abs_diff_eq!(v, gv, epsilon = EPSILON)
            );

        for triangle in value.iter() {
            let mut global_indices = [0usize; 3];

            for (vi, v) in triangle.iter().enumerate() {
                if let Some(global_index) = find_vertex(&vertices, &v) {
                    global_indices[vi] = global_index;
                } else {
                    vertices.push(*v);
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

impl TryFrom<Outline2D> for Geometry2D {
    type Error = anyhow::Error;

    fn try_from(value: Outline2D) -> Result<Self, Self::Error> {
        Ok(Self::from(Triangles::<P2>::try_from(value)?))
    }
}

/* =========== *
 * 3D geometry *
 * =========== */

 #[derive(Clone, Debug)]
pub struct Geometry3D {
    vertices: Vec<P3>,
    triangles: Vec<[usize; 3]>,
}

impl AsPrimitives<P3> for Geometry3D {
    fn from_primitives(vertices: Vec<P3>, triangles: Vec<[usize; 3]>) -> Self {
        Self {
            vertices,
            triangles,
        }
    }

    fn get_vertices(&self) -> &Vec<P3> {
        &self.vertices
    }

    fn get_triangles(&self) -> &Vec<[usize; 3]> {
        &self.triangles
    }
}

impl Geometry3D {
    pub fn as_manifold_edges(&self) -> Vec<[P3; 2]> {
        let vs = self.get_vertices();
        let ts = self.get_triangles();

        let mut edges = vec![];

        for [i0, i1, i2] in ts {
            if i0 < i1 {
                edges.push([vs[*i0].clone(), vs[*i1].clone()]);
            }
            if i1 < i2 {
                edges.push([vs[*i1].clone(), vs[*i2].clone()]);
            }
            if i2 < i0 {
                edges.push([vs[*i2].clone(), vs[*i0].clone()]);
            }
        }

        edges
    }

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

        let mesh = Self {
            vertices,
            triangles,
        };

        // TODO: validation

        mesh
    }
}
