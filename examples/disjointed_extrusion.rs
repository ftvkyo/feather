use feather::*;

use geometry::{Geometry2D, primitives::{Triangle, Triangles}};
use render::view::View;

fn main() {
    let ts = vec![
        Triangle::new((-1.0, 0.0), (-1.0, -1.0), (0.0, -1.0)),
        Triangle::new((1.0, 0.0), (1.0, 1.0), (0.0, 1.0)),
    ];

    let g2 = Geometry2D::from(Triangles::new(ts));

    let g3 = g2.extrude_linear(1.0);

    let view = View::new("disjointed_extrusion");
    view.run(g3);
}
