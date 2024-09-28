pub mod interface;
pub mod view;
pub mod wireframe;


use three_d::*;


pub fn material(context: &Context, albedo: Srgba) -> PhysicalMaterial {
    let mut material = PhysicalMaterial::new_opaque(
        &context,
        &CpuMaterial {
            albedo,
            ..Default::default()
        },
    );
    material.render_states.cull = Cull::Back;

    material
}
