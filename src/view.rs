use three_d::*;

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
        })
        .unwrap();

        Self {
            window,
            state: Default::default(),
        }
    }

    pub fn run(self, mesh: CpuMesh) {
        let clear_state = ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 100.0);

        let camera_distance = 3.0;
        let camera_position = vec3(camera_distance, 0.0, 0.0);
        let camera_target = vec3(0.0, 0.0, 0.0);
        let camera_up = vec3(0.0, 1.0, 0.0);

        /* ============= *
         * Context setup *
         * ============= */

        let context = self.window.gl();

        let mut camera = Camera::new_perspective(
            self.window.viewport(),
            camera_position,
            camera_target,
            camera_up,
            degrees(60.0),
            0.1,
            10.0,
        );
        let mut control = OrbitControl::new(camera_target, 1.0, 10.0);

        let light_ambient = AmbientLight::new(&context, 0.01, Srgba::WHITE);
        let mut light_camera = DirectionalLight::new(&context, 0.5, Srgba::WHITE, &(camera_target - camera_position));

        let mut model_material = PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::new_opaque(255, 255, 255),
                metallic: 0.25,
                roughness: 0.25,
                ..Default::default()
            },
        );
        model_material.render_states.cull = Cull::Back;
        let model = Gm::new(Mesh::new(&context, &mesh), model_material);

        let (edges, vertices) = crate::wireframe::generate_wireframe(&context, &mesh);

        /* ========= *
         * Rendering *
         * ========= */

        let View { window, mut state } = self;

        window.render_loop(move |mut frame_input| {
            let mut redraw = frame_input.first_frame;
            redraw |= camera.set_viewport(frame_input.viewport);
            redraw |= control.handle_events(&mut camera, &mut frame_input.events);
            redraw |= state.handle_events(&mut frame_input.events);

            if redraw {
                light_camera.direction = camera_target - camera.position();

                if !state.render_wireframe {
                    frame_input.screen().clear(clear_state).render(
                        &camera,
                        &model,
                        &[&light_ambient, &light_camera],
                    );
                } else {
                    frame_input.screen().clear(clear_state).render(
                        &camera,
                        model.into_iter().chain(&edges).chain(&vertices),
                        &[&light_ambient, &light_camera],
                    );
                }
            }

            FrameOutput {
                swap_buffers: redraw,
                ..Default::default()
            }
        });
    }
}
