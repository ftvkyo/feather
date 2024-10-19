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

impl<Point: Clone + std::fmt::Debug> IntoIterator for Triangle<Point> {
    type Item = Point;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.to_vec().into_iter()
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

impl<Point: Clone + std::fmt::Debug> IntoIterator for Triangles<Point> {
    type Item = Triangle<Point>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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

// FIXME: this API is not very nice
#[derive(Clone, Debug)]
pub struct Outline2D(pub Vec<P2>);

impl TryFrom<Outline2D> for Triangles<P2> {
    type Error = anyhow::Error;

    // FIXME: this does unconstrained Delaunay Triangulation which
    // means it's only valid for convex polygons.

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
