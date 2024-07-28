use three_d::*;
use hexasphere::shapes::IcoSphere;


pub fn make_sphere(subdivisions: usize) -> CpuMesh {
    let sphere = IcoSphere::new(subdivisions, |_| ());

    let positions = Positions::F32(sphere.raw_points().iter().map(|p| p.to_array().into()).collect());
    let indices = Indices::U32(sphere.get_all_indices());

    let mut mesh = CpuMesh {
        positions,
        indices,
        ..Default::default()
    };

    mesh.compute_normals();

    return mesh;
}
