// This file contains code from the `three-d` library examples licensed under MIT.
// Source: https://github.com/asny/three-d/blob/a69b70874d93da231f5e4f8c35adb4d38134aeee/examples/wireframe/src/main.rs

use three_d::*;

use crate::Geometry3D;

pub type Wireframe = Gm<InstancedMesh, PhysicalMaterial>;

pub fn generate_wireframe(context: &Context, geometry: &Geometry3D) -> (Wireframe, Wireframe) {
    let mut wireframe_material = PhysicalMaterial::new_opaque(
        &context,
        &CpuMaterial {
            albedo: Srgba::new_opaque(220, 50, 50),
            ..Default::default()
        },
    );
    wireframe_material.render_states.cull = Cull::Back;

    let scale = 0.007;

    let mut cylinder = CpuMesh::cylinder(10);
    cylinder
        .transform(&Mat4::from_nonuniform_scale(1.0, scale, scale))
        .unwrap();
    let edges = Gm::new(
        InstancedMesh::new(&context, &edge_transformations(&geometry), &cylinder),
        wireframe_material.clone(),
    );

    let mut sphere = CpuMesh::sphere(8);
    sphere.transform(&Mat4::from_scale(scale)).unwrap();
    let vertices = Gm::new(
        InstancedMesh::new(&context, &vertex_transformations(&geometry), &sphere),
        wireframe_material,
    );

    (edges, vertices)
}

fn edge_transformations(geometry: &Geometry3D) -> Instances {
    let positions = &geometry.positions;
    let indices = &geometry.indices;

    let mut transformations = Vec::new();

    for f in 0..indices.len() / 3 {
        let i1 = indices[3 * f] as usize;
        let i2 = indices[3 * f + 1] as usize;
        let i3 = indices[3 * f + 2] as usize;

        if i1 < i2 {
            transformations.push(edge_transform(positions[i1], positions[i2]));
        }
        if i2 < i3 {
            transformations.push(edge_transform(positions[i2], positions[i3]));
        }
        if i3 < i1 {
            transformations.push(edge_transform(positions[i3], positions[i1]));
        }
    }

    Instances {
        transformations,
        ..Default::default()
    }
}

fn edge_transform(p1: Vector3<f64>, p2: Vector3<f64>) -> Mat4 {
    let p1 = p1.cast().unwrap();
    let p2 = p2.cast().unwrap();

    Mat4::from_translation(p1)
        * Into::<Mat4>::into(Quat::from_arc(
            vec3(1.0, 0.0, 0.0),
            (p2 - p1).normalize(),
            None,
        ))
        * Mat4::from_nonuniform_scale((p1 - p2).magnitude(), 1.0, 1.0)
}

fn vertex_transformations(geometry: &Geometry3D) -> Instances {
    Instances {
        transformations: geometry
            .positions
            .iter()
            .map(|v| v.cast::<f32>().unwrap())
            .map(Matrix4::from_translation)
            .collect(),
        ..Default::default()
    }
}
