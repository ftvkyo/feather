use feather::{App, make_sphere};


fn main() {
    let app = App::new("Sphere");

    let sphere = make_sphere(1);
    app.run(sphere);
}
