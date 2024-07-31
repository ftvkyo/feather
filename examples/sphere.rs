use feather::{App, Mesh3D};

fn main() {
    let app = App::new("Sphere");

    let sphere = Mesh3D::sphere(1);
    app.run(sphere);
}
