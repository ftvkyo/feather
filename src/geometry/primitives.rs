/// Used floating point type
pub type FP = f64;

/// Comparison of floats
pub const EPSILON: FP = 0.000_000_1;

pub type P2 = cgmath::Point2<FP>;
pub type V2 = cgmath::Vector2<FP>;

pub type P3 = cgmath::Point3<FP>;
pub type V3 = cgmath::Vector3<FP>;

pub fn spade_from_p2(p: P2) -> spade::Point2<FP> {
    spade::Point2::new(p.x, p.y)
}

pub fn spade_to_p2(p: spade::Point2<FP>) -> P2 {
    P2::new(p.x, p.y)
}

/// A single triangle
#[derive(Clone, Debug)]
pub struct Triangle<Point: Clone + std::fmt::Debug>([Point; 3]);

impl<Point: Clone + std::fmt::Debug> std::ops::Index<usize> for Triangle<Point> {
    type Output = Point;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<Point: Clone + std::fmt::Debug> std::ops::IndexMut<usize> for Triangle<Point> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<Point: Clone + std::fmt::Debug> Triangle<Point> {
    // TODO: check that area is non-zero

    pub fn new<P: Into<Point>>(p1: P, p2: P, p3: P) -> Self {
        Self([
            p1.into(),
            p2.into(),
            p3.into(),
        ])
    }

    pub fn from_points(points: [Point; 3]) -> Self {
        Self(points)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Point> {
        self.0.iter()
    }
}

/// A collection of triangles
#[derive(Clone, Debug)]
pub struct Triangles<Point: Clone + std::fmt::Debug>(Vec<Triangle<Point>>);

impl<Point: Clone + std::fmt::Debug> std::ops::Index<usize> for Triangles<Point> {
    type Output = Triangle<Point>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<Point: Clone + std::fmt::Debug> std::ops::IndexMut<usize> for Triangles<Point> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<Point: Clone + std::fmt::Debug> Triangles<Point> {
    pub fn new(triangles: Vec<Triangle<Point>>) -> Self {
        Self(triangles)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Triangle<Point>> {
        self.0.iter()
    }
}

#[derive(Clone, Debug)]
pub struct Outline2D(pub Vec<P2>);

impl TryFrom<Outline2D> for Triangles<P2> {
    type Error = anyhow::Error;

    fn try_from(value: Outline2D) -> Result<Self, Self::Error> {
        use spade::Triangulation;

        let mut tri = spade::DelaunayTriangulation::<spade::Point2<FP>>::new();

        for point in value.0 {
            tri.insert(spade_from_p2(point))?;
        }

        let triangles: Vec<_> = tri.inner_faces()
            .map(|f| Triangle(f.vertices().map(|v| spade_to_p2(*v.data()))))
            .collect();

        Ok(Self(triangles))
    }
}

pub trait AsPrimitives<Point: Clone + std::fmt::Debug> {
    fn from_primitives(vertices: Vec<Point>, triangles: Vec<[usize; 3]>) -> Self;

    fn get_vertices(&self) -> &Vec<Point>;
    fn get_triangles(&self) -> &Vec<[usize; 3]>;

    fn as_vertices(&self) -> Vec<Point> {
        let vs = self.get_vertices();
        let ts = self.get_triangles();

        let mut vertices = vec![];

        for [t0, t1, t2] in ts {
            vertices.push(vs[*t0].clone());
            vertices.push(vs[*t1].clone());
            vertices.push(vs[*t2].clone());
        }

        vertices
    }

    fn as_triangles(&self) -> Triangles<Point> {
        let vs = self.get_vertices();
        let ts = self.get_triangles();

        let mut triangles = vec![];

        for [t0, t1, t2] in ts {
            triangles.push(Triangle::from_points([
                vs[*t0].clone(),
                vs[*t1].clone(),
                vs[*t2].clone(),
            ]));
        }

        Triangles::new(triangles)
    }

    /// Returns edges that are used by an odd number of triangles.
    /// For 2D objects, this is their outlines and holes.
    /// For 3D objects, this is what would make them non-manifold.
    fn as_outer_edge_indices(&self) -> Vec<[usize; 2]> {
        use std::collections::BTreeSet;

        let ts = self.get_triangles();

        let mut edges: BTreeSet<[usize; 2]> = BTreeSet::new();

        for [t0, t1, t2] in ts {
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
}
