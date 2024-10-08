use three_d::*;

pub type Axes = Gm<InstancedMesh, PhysicalMaterial>;

pub fn generate_axes(context: &Context, height: f32) -> Axes {
    let material = super::material(context, Srgba::new_opaque(200, 200, 200));

    let scale = 0.005;

    let mut base = CpuMesh::cylinder(12);
    base.transform(&Mat4::from_nonuniform_scale(height, scale, scale)).unwrap();
    base.transform(&Mat4::from_translation(vec3(- height / 2.0, 0.0, 0.0))).unwrap();

    let x = Mat4::identity();
    let y = Mat4::from_angle_z(Deg(90.0));
    let z = Mat4::from_angle_y(Deg(90.0));

    let instances = Instances {
        transformations: vec![x, y, z],
        colors: Some(vec![
            Srgba::RED,
            Srgba::GREEN,
            Srgba::BLUE,
        ]),
        ..Default::default()
    };

    let axes = Gm::new(
        InstancedMesh::new(&context, &instances, &base),
        material,
    );

    axes
}
