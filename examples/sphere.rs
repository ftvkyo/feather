use feather::{App, Polyhedron};

fn main() {
    let app = App::new("Sphere");

    let sphere = Polyhedron::sphere(1);
    app.run(sphere);
}
