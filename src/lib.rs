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
        let camera_target = vec3(0.0, 0.0, 0.0);
        let camera_up = vec3(0.0, 1.0, 0.0);

        let mut camera = Camera::new_perspective(
            self.window.viewport(),
            vec3(3.0, 0.0, 0.0),
            camera_target,
            camera_up,
            degrees(60.0),
            0.1,
            10.0,
        );

        let context = self.window.gl();
        let model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());

        let angle_per_second = 0.1f32; // in radians

        self.window.render_loop(move |frame_input| {
            camera.set_viewport(frame_input.viewport);

            let secs = frame_input.elapsed_time as f32 / 1_000.0;
            let angle = angle_per_second * secs;
            let rotation = Matrix3::from_cols(
                vec3(angle.cos(), - angle.sin(), 0.0),
                vec3(angle.sin(), angle.cos(), 0.0),
                vec3(0.0, 0.0, 1.0),
            );

            // TODO: Look into OrbitControl

            let camera_position = <Matrix3<_> as Transform<Point3<_>>>::transform_vector(&rotation, camera.position().clone());

            camera.set_view(
                camera_position,
                camera_target,
                camera_up,
            );

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

    CpuMesh {
        positions,
        indices,
        colors,
        ..Default::default()
    }
}
