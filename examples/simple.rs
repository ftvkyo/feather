use feather::{App, make_sphere};


fn main() {
    let app = App::new("Simple");

    let sphere = make_sphere(0);
    app.run(sphere);
}
