use hexasphere::{shapes::IcoSphere, BaseShape, Subdivided};
use stl_io::{IndexedMesh, IndexedTriangle, Normal, Triangle, Vertex};
use three_d::{vec3, CpuMesh, Indices, Positions, Vec3};


fn make_indexed_mesh<T, S: BaseShape>(subdivided: Subdivided<T, S>) -> IndexedMesh {
    let vertices: Vec<Vec3> = subdivided.raw_points().iter().map(|p| p.to_array().into()).collect();
    let faces = subdivided.get_all_indices().chunks(3).map(|face| {
        let face = [
            face[0] as usize,
            face[1] as usize,
            face[2] as usize,
        ];

        let a = vertices[face[1]] - vertices[face[0]];
        let b = vertices[face[2]] - vertices[face[0]];

        let n = a.cross(b);

        let normal = Normal::new(n.into());

        IndexedTriangle {
            normal,
            vertices: face,
        }
    }).collect();

    let mesh = IndexedMesh {
        vertices: vertices.into_iter().map(|v| Vertex::new(v.into())).collect(),
        faces,
    };

    mesh.validate().unwrap();

    mesh
}


pub fn make_sphere(subdivisions: usize) -> IndexedMesh {
    let sphere = IcoSphere::new(subdivisions, |_| ());
    return make_indexed_mesh(sphere)
}


pub(crate) fn make_cpu_mesh(mesh: IndexedMesh) -> CpuMesh {
    let positions: Vec<_> = mesh.vertices.iter().map(|v| vec3(v[0], v[1], v[2])).collect();
    let indices: Vec<_> = mesh.faces.iter().flat_map(|f| f.vertices).map(|i| i as u32).collect();

    let mut mesh = CpuMesh {
        positions: Positions::F32(positions),
        indices: Indices::U32(indices),
        ..Default::default()
    };

    mesh.compute_normals();

    mesh
}


pub fn produce_stl(mesh: IndexedMesh) -> Vec<Triangle> {
    let IndexedMesh {
        vertices,
        faces,
    } = mesh;

    return faces
        .iter()
        .map(|face| {
            let normal = face.normal;
            let vertices = [
                vertices[face.vertices[0]],
                vertices[face.vertices[1]],
                vertices[face.vertices[2]],
            ];
            Triangle {
                normal,
                vertices,
            }
        })
        .collect();
}
