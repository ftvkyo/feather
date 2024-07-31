use feather::{App, Polygon};

fn main() {
    let app = App::new("Triangle");

    let triangle = Polygon::circle(3);
    app.run(triangle.extrude_linear(0.33));
}
