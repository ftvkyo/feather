use std::io::{BufWriter, Write};

use byteorder::{LittleEndian, WriteBytesExt};
use cgmath::{prelude::*, vec2, Basis2, Point2, Point3, Rad, Vector2, Vector3};
use geo::TriangulateSpade;
use three_d::{CpuMesh, Indices, Positions};

type F = f64;
type P2D = Point2<F>;
type V2D = Vector2<F>;
type P3D = Point3<F>;
type V3D = Vector3<F>;

pub struct Geometry2D {
    inner: geo::Polygon,
}

impl Geometry2D {
    pub fn circle(sides: usize) -> Self {
        assert!(sides >= 3, "Sides ({sides}) should be >= 3");

        let origin = Point2::new(0.0, 0.0);
        let rot: Basis2<F> = Rotation2::from_angle(Rad::full_turn() / sides as F);

        let mut v = vec2(1.0, 0.0);
        let mut positions: Vec<[F; 2]> = vec![];

        for _ in 0..sides as u32 {
            let position = origin + v;
            positions.push(position.into());
            v = rot.rotate_vector(v);
        }

        let mut linestring: geo::LineString = positions.into();
        linestring.close();

        Self {
            inner: geo::Polygon::new(linestring, vec![]),
        }
    }
}

impl Geometry2D {
    pub fn extrude_linear(&self, extent: F) -> Geometry3D {
        let extent = extent / 2.0; // Go half down and half up (like `center = true` in OpenSCAD)
        let triangulation = self.inner.constrained_triangulation(Default::default()).unwrap();

        let mut positions = vec![];
        let mut indices = vec![];

        let mut add_pos = |new_pos| if let Some(index) = positions.iter().position(|pos| pos == &new_pos) {
            index
        } else {
            positions.push(new_pos);
            positions.len() - 1
        };

        let mut add_triangle = |t: [V3D; 3]| {
            indices.push(add_pos(t[0]) as u32);
            indices.push(add_pos(t[1]) as u32);
            indices.push(add_pos(t[2]) as u32);
        };

        // Add all triangles

        for triangle in triangulation {
            // Top triangle
            let triangle1 = [
                V3D::new(triangle.0.x, triangle.0.y, extent),
                V3D::new(triangle.1.x, triangle.1.y, extent),
                V3D::new(triangle.2.x, triangle.2.y, extent),
            ];

            add_triangle(triangle1);

            // Bottom triangle
            let triangle2 = [
                V3D::new(triangle.2.x, triangle.2.y, - extent),
                V3D::new(triangle.1.x, triangle.1.y, - extent),
                V3D::new(triangle.0.x, triangle.0.y, - extent),
            ];

            add_triangle(triangle2);
        }

        // Add the exterior

        let exterior = self.inner.exterior();
        for edge in exterior.lines() {
            // Top triangle
            let triangle1 = [
                V3D::new(edge.start.x, edge.start.y, extent),
                V3D::new(edge.start.x, edge.start.y, - extent),
                V3D::new(edge.end.x, edge.end.y, - extent),
            ];

            add_triangle(triangle1);

            // Bottom triangle
            let triangle2 = [
                V3D::new(edge.start.x, edge.start.y, extent),
                V3D::new(edge.end.x, edge.end.y, - extent),
                V3D::new(edge.end.x, edge.end.y, extent),
            ];

            add_triangle(triangle2);
        }

        if self.inner.num_interior_rings() != 0 {
            unimplemented!("Extrusion of polygons with interior rings");
        }

        Geometry3D {
            positions,
            indices,
        }
    }
}

pub struct Geometry3D {
    positions: Vec<V3D>,
    indices: Vec<u32>,
}

impl Geometry3D {
    pub fn sphere(subdivisions: usize) -> Self {
        let subdivided = hexasphere::shapes::IcoSphere::new(subdivisions, |_| ());

        let positions: Vec<V3D> = subdivided
            .raw_points()
            .iter()
            .map(|p| p.to_array().map(|c| c as F).into())
            .collect();

        let indices: Vec<_> = subdivided
            .get_all_indices()
            .clone();

        let mesh = Self {
            positions,
            indices,
        };

        // TODO: validation

        mesh
    }
}

impl Geometry3D {
    pub fn stl<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let mut writer = BufWriter::new(writer);

        // Write 80-byte header (all zeros)
        writer.write_all(&[0u8; 80])?;

        // Write 4-byte number of triangles
        writer.write_u32::<LittleEndian>(self.indices.len() as u32 / 3)?;

        // For each triangle
        for face in self.indices.chunks(3) {

            // Calculate normal
            let a = self.positions[face[1] as usize] - self.positions[face[0] as usize];
            let b = self.positions[face[2] as usize] - self.positions[face[0] as usize];
            let normal = a.cross(b);

            // Write 3 x 4-byte normal
            writer.write_f32::<LittleEndian>(normal.x as f32)?;
            writer.write_f32::<LittleEndian>(normal.y as f32)?;
            writer.write_f32::<LittleEndian>(normal.z as f32)?;

            // For each point in the triangle
            for pindex in face {
                let position = self.positions[*pindex as usize];

                // Write 3 x 4-byte point
                writer.write_f32::<LittleEndian>(position.x as f32)?;
                writer.write_f32::<LittleEndian>(position.y as f32)?;
                writer.write_f32::<LittleEndian>(position.z as f32)?;
            }

            // We don't use any memory for attributes
            writer.write_u16::<LittleEndian>(0)?;
        }

        writer.flush()
    }
}

impl Into<CpuMesh> for Geometry3D {
    fn into(self) -> CpuMesh {
        let mut mesh = CpuMesh {
            positions: Positions::F64(self.positions),
            indices: Indices::U32(self.indices),
            ..Default::default()
        };

        mesh.compute_normals();

        mesh
    }
}
