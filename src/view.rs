use stl_io::IndexedMesh;
use three_d::*;

use crate::make_cpu_mesh;

struct ViewState {
    pub render_wireframe: bool,
}

impl ViewState {
    pub fn handle_events(&mut self, events: &mut [Event]) -> bool {
        let mut change = false;
        for event in events.iter_mut() {
            match event {
                Event::KeyPress {
                    kind: Key::D,
                    handled,
                    ..
                } => {
                    self.render_wireframe = !self.render_wireframe;
                    *handled = true;
                    change = true;
                },
                _ => {},
            }
        }
        change
    }
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            render_wireframe: false,
        }
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
        }).unwrap();

        Self {
            window,
            state: Default::default(),
        }
    }

    pub fn run(self, mesh: IndexedMesh) {
        let cpu_mesh = make_cpu_mesh(mesh);

        let camera_distance = 3.0;
        let camera_target = vec3(0.0, 0.0, 0.0);
        let camera_up = vec3(0.0, 1.0, 0.0);

        let mut camera = Camera::new_perspective(
            self.window.viewport(),
            vec3(camera_distance, 0.0, 0.0),
            camera_target,
            camera_up,
            degrees(60.0),
            0.1,
            10.0,
        );
        let mut control = OrbitControl::new(
            camera_target,
            1.0,
            10.0,
        );

        let context = self.window.gl();

        let mut model_material = PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::new_opaque(50, 50, 50),
                ..Default::default()
            },
        );
        model_material.render_states.cull = Cull::Back;
        let model = Gm::new(Mesh::new(&context, &cpu_mesh), model_material);

        let (edges, vertices) = crate::wireframe::generate_wireframe(&context, &cpu_mesh);

        let clear = ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 100.0);

        let ambient = AmbientLight::new(&context, 0.7, Srgba::WHITE);
        let directional0 = DirectionalLight::new(&context, 2.0, Srgba::WHITE, &vec3(-1.0, -1.0, -1.0));
        let directional1 = DirectionalLight::new(&context, 2.0, Srgba::WHITE, &vec3(1.0, 1.0, 1.0));

        let View {
            window,
            mut state,
        } = self;

        window.render_loop(move |mut frame_input| {
            let mut redraw = frame_input.first_frame;
            redraw |= camera.set_viewport(frame_input.viewport);
            redraw |= control.handle_events(&mut camera, &mut frame_input.events);
            redraw |= state.handle_events(&mut frame_input.events);

            if redraw {
                if !state.render_wireframe {
                    frame_input
                        .screen()
                        .clear(clear)
                        .render(&camera, &model, &[&ambient, &directional0, &directional1]);
                } else {
                    frame_input
                        .screen()
                        .clear(clear)
                        .render(&camera, model.into_iter().chain(&edges).chain(&vertices), &[&ambient, &directional0, &directional1]);
                }
            }

            FrameOutput {
                swap_buffers: redraw,
                ..Default::default()
            }
        });
    }
}
