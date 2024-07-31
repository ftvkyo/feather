use std::io::{BufWriter, Write};

use byteorder::{LittleEndian, WriteBytesExt};
use three_d::*;

pub struct Mesh2D {
    positions: Vec<Point2<f32>>,
    indices: Vec<usize>,
}

impl Mesh2D {
    pub fn circle(sides: usize) -> Self {
        assert!(sides >= 3, "Sides ({sides}) should be >= 3");

        let origin = Point2::new(0.0, 0.0);
        let rot = Matrix2::from_angle(Rad::full_turn() / sides as f32);

        let mut v = vec2(1.0, 0.0);
        let mut positions = vec![origin];
        let mut indices = vec![];

        for i in 0..sides {
            positions.push(origin + v);
            indices.extend([0, i, i + 1]);

            v = rot * v;
        }

        let mesh = Self {
            positions,
            indices,
        };

        // TODO: validation

        mesh
    }
}

impl Into<CpuMesh> for Mesh2D {
    fn into(self) -> CpuMesh {
        todo!();
    }
}

pub struct Mesh3D {
    positions: Vec<Vector3<f32>>,
    indices: Vec<u32>,
}

impl Mesh3D {
    pub fn sphere(subdivisions: usize) -> Self {
        let subdivided = hexasphere::shapes::IcoSphere::new(subdivisions, |_| ());

        let positions: Vec<Vector3<_>> = subdivided
            .raw_points()
            .iter()
            .map(|p| p.to_array().into())
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

impl Mesh3D {
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
            writer.write_f32::<LittleEndian>(normal.x)?;
            writer.write_f32::<LittleEndian>(normal.y)?;
            writer.write_f32::<LittleEndian>(normal.z)?;

            // For each point in the triangle
            for pindex in face {
                let position = self.positions[*pindex as usize];

                // Write 3 x 4-byte point
                writer.write_f32::<LittleEndian>(position.x)?;
                writer.write_f32::<LittleEndian>(position.y)?;
                writer.write_f32::<LittleEndian>(position.z)?;
            }

            // We don't use any memory for attributes
            writer.write_u16::<LittleEndian>(0)?;
        }

        writer.flush()
    }
}

impl Into<CpuMesh> for Mesh3D {
    fn into(self) -> CpuMesh {
        let mut mesh = CpuMesh {
            positions: Positions::F32(self.positions),
            indices: Indices::U32(self.indices),
            ..Default::default()
        };

        mesh.compute_normals();

        mesh
    }
}
