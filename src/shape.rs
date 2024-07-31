use stl_io::{IndexedMesh, IndexedTriangle, Normal, Vertex};
use three_d::{vec3, CpuMesh, Indices, Point2, Positions, Vec3};

pub trait Shape {
    fn mesh(&self) -> IndexedMesh;

    fn mesh_render(&self) -> CpuMesh {
        let mesh = self.mesh();

        let positions: Vec<_> = mesh
            .vertices
            .iter()
            .map(|v| vec3(v[0], v[1], v[2]))
            .collect();
        let indices: Vec<_> = mesh
            .faces
            .iter()
            .flat_map(|f| f.vertices)
            .map(|i| i as u32)
            .collect();

        let mut mesh = CpuMesh {
            positions: Positions::F32(positions),
            indices: Indices::U32(indices),
            ..Default::default()
        };

        mesh.compute_normals();

        mesh
    }
}

pub enum Polygon {
    Regular { sides: usize },
    Simple { outline: Vec<Point2<f32>> },
}

impl Shape for Polygon {
    fn mesh(&self) -> IndexedMesh {
        todo!()
    }

    fn mesh_render(&self) -> CpuMesh {
        todo!()
    }
}

pub enum Polyhedron {
    Cube,
    Cylinder { subdivisions: usize },
    Sphere { subdivisions: usize },
}

impl Polyhedron {
    pub fn cube() -> Self {
        Self::Cube
    }

    pub fn cylinder(subdivisions: usize) -> Self {
        Self::Cylinder { subdivisions }
    }

    pub fn sphere(subdivisions: usize) -> Self {
        Self::Sphere { subdivisions }
    }
}

impl Shape for Polyhedron {
    fn mesh(&self) -> IndexedMesh {
        match self {
            Self::Sphere { subdivisions } => {
                let subdivided = hexasphere::shapes::IcoSphere::new(*subdivisions, |_| ());

                let vertices: Vec<Vec3> = subdivided
                    .raw_points()
                    .iter()
                    .map(|p| p.to_array().into())
                    .collect();
                let faces = subdivided
                    .get_all_indices()
                    .chunks(3)
                    .map(|face| {
                        let face = [face[0] as usize, face[1] as usize, face[2] as usize];

                        let a = vertices[face[1]] - vertices[face[0]];
                        let b = vertices[face[2]] - vertices[face[0]];

                        let n = a.cross(b);

                        let normal = Normal::new(n.into());

                        IndexedTriangle {
                            normal,
                            vertices: face,
                        }
                    })
                    .collect();

                let mesh = IndexedMesh {
                    vertices: vertices
                        .into_iter()
                        .map(|v| Vertex::new(v.into()))
                        .collect(),
                    faces,
                };

                mesh.validate().unwrap();

                mesh
            }
            _ => todo!(),
        }
    }
}

// TODO: only implement STL export for Polyhedra
