use stl_io::{IndexedMesh, Triangle};

pub fn produce_stl(mesh: IndexedMesh) -> Vec<Triangle> {
    let IndexedMesh { vertices, faces } = mesh;

    return faces
        .iter()
        .map(|face| {
            let normal = face.normal;
            let vertices = [
                vertices[face.vertices[0]],
                vertices[face.vertices[1]],
                vertices[face.vertices[2]],
            ];
            Triangle { normal, vertices }
        })
        .collect();
}
