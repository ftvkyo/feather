use three_d::*;
use three_d_asset::ProjectionType;

use crate::geometry::{Geometry2D, Geometry3D};

use super::{interface::generate_axes, wireframe::generate_wireframe};


const DEFAULT_FOV: Deg<f32> = Deg(60.0);
const DEFAULT_DISTANCE: f32 = 5.0;
const DEFAULT_Z_NEAR: f32 = 0.1;
// Setting this to f32::MAX / 2.0 breaks depth buffer for orthographic projection :)
const DEFAULT_Z_FAR: f32 = 100.0;


impl Into<CpuMesh> for Geometry3D {
    fn into(self) -> CpuMesh {
        // `vertices` and `triangles` could just be copied, however, the three-d crate doesn't support flat rendering.
        // So, to achieve flat rendering, there is no index buffer used.

        let vertices = self.iter_vertices().map(Point3::to_vec).collect();

        let mut mesh = CpuMesh {
            positions: Positions::F64(vertices),
            ..Default::default()
        };

        mesh.compute_normals();

        mesh
    }
}

impl Into<CpuMesh> for Geometry2D {
    fn into(self) -> CpuMesh {
        let g3 = self.extrude_linear(0.1);
        g3.into()
    }
}

struct ViewState {
    pub should_exit: bool,
    pub render_wireframe: bool,
}

impl ViewState {
    pub fn handle_events(&mut self, camera: &mut Camera, events: &mut [Event]) -> bool {
        let mut change = false;
        for event in events.iter_mut() {
            match event {
                Event::KeyPress {
                    kind: Key::Q,
                    handled,
                    ..
                } => {
                    self.should_exit = true;
                    *handled = true;
                    change = true;
                }
                Event::KeyPress {
                    kind: Key::D,
                    handled,
                    ..
                } => {
                    self.render_wireframe = !self.render_wireframe;
                    *handled = true;
                    change = true;
                }
                Event::KeyPress {
                    kind: Key::P,
                    handled,
                    ..
                } => {
                    match camera.projection_type() {
                        ProjectionType::Orthographic { .. } => {
                            camera.set_perspective_projection(DEFAULT_FOV, DEFAULT_Z_NEAR, DEFAULT_Z_FAR);
                        }
                        ProjectionType::Perspective { .. } => {
                            let height = camera.position().magnitude();
                            camera.set_orthographic_projection(height, DEFAULT_Z_NEAR, DEFAULT_Z_FAR);
                        }
                    }

                    *handled = true;
                    change = true;
                }
                _ => {}
            }
        }
        change
    }
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            should_exit: false,
            render_wireframe: false,
        }
    }
}

pub struct Lights {
    spread: Rad<f32>,
    strength: f32,

    ambient: AmbientLight,
    r: DirectionalLight,
    g: DirectionalLight,
    b: DirectionalLight,
}

impl Lights {
    pub fn new(context: &Context, direction: Vector3<f32>, strength: f32, spread: Rad<f32>) -> Self {
        // To put the first light into the required position
        let rot1 = Matrix4::from_angle_z(spread);
        // To put the next lights relative to the previous lights
        let rot2 = Matrix4::from_axis_angle(direction, Rad::full_turn() / 3.0);

        let vr = rot1.transform_vector(direction);
        let vg = rot2.transform_vector(vr.clone());
        let vb = rot2.transform_vector(vg.clone());

        let r = DirectionalLight::new(&context, strength, Srgba::RED, &vr);
        let g = DirectionalLight::new(&context, strength, Srgba::GREEN, &vg);
        let b = DirectionalLight::new(&context, strength, Srgba::BLUE, &vb);

        let ambient = AmbientLight::new(&context, 0.1, Srgba::WHITE);

        Self {
            strength,
            spread,

            ambient,
            r,
            g,
            b,
        }
    }

    pub fn update(&mut self, context: &Context, direction: Vector3<f32>) {
        *self = Self::new(context, direction, self.strength, self.spread);
    }
}

pub struct View {
    window: Window,
    state: ViewState,
}

impl View {
    pub fn new<S: ToString>(title: S) -> Self {
        let window = Window::new(WindowSettings {
            title: title.to_string(),
            ..Default::default()
        })
        .unwrap();

        Self {
            window,
            state: Default::default(),
        }
    }

    pub fn run(self, geometry: Geometry3D) {
        let clear_state = ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0);

        let camera_position = vec3(DEFAULT_DISTANCE, 0.0, 0.0);
        let camera_target = vec3(0.0, 0.0, 0.0);
        let camera_up = vec3(0.0, 1.0, 0.0);

        /* ============= *
         * Context setup *
         * ============= */

        let context = self.window.gl();

        let mut camera = Camera::new_orthographic(
            self.window.viewport(),
            camera_position,
            camera_target,
            camera_up,
            DEFAULT_DISTANCE,
            DEFAULT_Z_NEAR,
            DEFAULT_Z_FAR,
        );

        let mut control = OrbitControl::new(camera_target, 1.0, 10.0);

        let mut lights = Lights::new(&context, camera_target - camera_position, 0.5, Deg(45.0).into());

        let model_material = super::material(&context, Srgba::WHITE);

        let axes = generate_axes(&context, 100.0);
        let (edges, vertices) = generate_wireframe(&context, &geometry);
        let model = Gm::new(Mesh::new(&context, &geometry.into()), model_material);


        /* ========= *
         * Rendering *
         * ========= */

        let View { window, mut state } = self;

        window.render_loop(move |mut frame_input| {
            let mut redraw = frame_input.first_frame;
            redraw |= camera.set_viewport(frame_input.viewport);
            redraw |= control.handle_events(&mut camera, &mut frame_input.events);
            redraw |= state.handle_events(&mut camera, &mut frame_input.events);

            if redraw {
                lights.update(&context, camera_target - camera.position());

                let mut objects: Vec<&dyn Object> = vec![&model, &axes];

                if state.render_wireframe {
                    objects.push(&edges);
                    objects.push(&vertices);
                }

                frame_input.screen().clear(clear_state).render(
                    &camera,
                    &objects,
                    &[&lights.ambient, &lights.r, &lights.g, &lights.b],
                );
            }

            FrameOutput {
                swap_buffers: redraw,
                exit: state.should_exit,
                ..Default::default()
            }
        });
    }
}
