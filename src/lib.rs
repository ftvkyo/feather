use hexasphere::shapes::IcoSphere;
use three_d::*;


pub struct App {
    window: Window,
}


impl App {
    pub fn new<S: ToString>(title: S) -> Self {
        let window = Window::new(WindowSettings {
            title: title.to_string(),
            ..Default::default()
        }).unwrap();

        Self {
            window,
        }
    }

    pub fn run(self, cpu_mesh: CpuMesh) {
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
        let model = Gm::new(Mesh::new(&context, &cpu_mesh), NormalMaterial::default());

        self.window.render_loop(move |mut frame_input| {
            camera.set_viewport(frame_input.viewport);
            control.handle_events(&mut camera, &mut frame_input.events);

            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 100.0))
                .render(&camera, &model, &[]);

            FrameOutput::default()
        });
    }
}


pub fn make_sphere(subdivisions: usize) -> CpuMesh {
    let sphere = IcoSphere::new(subdivisions, |_| ());

    let positions = Positions::F32(sphere.raw_points().iter().map(|p| p.to_array().into()).collect());
    let indices = Indices::U32(sphere.get_all_indices());
    let colors = Some((0..positions.len()).map(|_| Srgba::new(255, 0, 0, 255)).collect());

    let mut mesh = CpuMesh {
        positions,
        indices,
        colors,
        ..Default::default()
    };

    mesh.compute_normals();

    return mesh;
}
