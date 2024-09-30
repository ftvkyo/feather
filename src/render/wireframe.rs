// This file contains code from the `three-d` library examples licensed under MIT.
// Source: https://github.com/asny/three-d/blob/a69b70874d93da231f5e4f8c35adb4d38134aeee/examples/wireframe/src/main.rs

use three_d::*;

use crate::geometry::{primitives::P3, Geometry3D};

pub type Wireframe = Gm<InstancedMesh, PhysicalMaterial>;

pub fn generate_wireframe(context: &Context, geometry: &Geometry3D) -> (Wireframe, Wireframe) {
    let material = super::material(context, Srgba::new_opaque(200, 200, 100));

    let scale = 0.007;

    let mut cylinder = CpuMesh::cylinder(10);
    cylinder
        .transform(&Mat4::from_nonuniform_scale(1.0, scale, scale))
        .unwrap();
    let edges = Gm::new(
        InstancedMesh::new(&context, &edge_transformations(&geometry), &cylinder),
        material.clone(),
    );

    let mut sphere = CpuMesh::sphere(8);
    sphere.transform(&Mat4::from_scale(scale)).unwrap();
    let vertices = Gm::new(
        InstancedMesh::new(&context, &vertex_transformations(&geometry), &sphere),
        material,
    );

    (edges, vertices)
}

fn edge_transformations(geometry: &Geometry3D) -> Instances {
    let mut transformations = Vec::new();

    let edges = geometry.as_manifold_edges();
    for [v0, v1] in edges {
        transformations.push(edge_transform(v0, v1));
    }

    Instances {
        transformations,
        ..Default::default()
    }
}

fn edge_transform(p1: P3, p2: P3) -> Mat4 {
    let p1 = p1.to_vec().cast().unwrap();
    let p2 = p2.to_vec().cast().unwrap();

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
            .iter_vertices()
            .map(|v| v.to_vec().cast::<f32>().unwrap())
            .map(Matrix4::from_translation)
            .collect(),
        ..Default::default()
    }
}
