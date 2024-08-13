use feather::{App, Geometry2D};

fn main() {
    let app = App::new("Triangle");

    let triangle = Geometry2D::circle(3);
    app.run(triangle.extrude_linear(0.33));
}
