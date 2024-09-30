use std::io::{BufWriter, Write};
use byteorder::{LittleEndian, WriteBytesExt};

use crate::geometry::primitives::{Triangles, P3};

impl Triangles<P3> {
    pub fn stl<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let mut writer = BufWriter::new(writer);

        // Write 80-byte header (all zeros)
        writer.write_all(&[0u8; 80])?;

        // Write 4-byte number of triangles
        writer.write_u32::<LittleEndian>(self.iter().len() as u32)?;

        // For each triangle
        for triangle in self.iter() {
            // Calculate normal
            let a = triangle[1] - triangle[0];
            let b = triangle[2] - triangle[0];
            let normal = a.cross(b);

            // Write 3 x 4-byte normal
            writer.write_f32::<LittleEndian>(normal.x as f32)?;
            writer.write_f32::<LittleEndian>(normal.y as f32)?;
            writer.write_f32::<LittleEndian>(normal.z as f32)?;

            // For each point in the triangle
            for vertex in triangle.iter() {
                // Write 3 x 4-byte point
                writer.write_f32::<LittleEndian>(vertex.x as f32)?;
                writer.write_f32::<LittleEndian>(vertex.y as f32)?;
                writer.write_f32::<LittleEndian>(vertex.z as f32)?;
            }

            // We don't use any memory for attributes
            writer.write_u16::<LittleEndian>(0)?;
        }

        writer.flush()
    }
}
