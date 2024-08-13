use feather::{App, Geometry3D};

fn main() {
    let app = App::new("Sphere");

    let sphere = Geometry3D::sphere(1);
    app.run(sphere);
}
