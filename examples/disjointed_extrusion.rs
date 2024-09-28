use feather::*;

use geometry::{Triangles, Geometry2D};
use render::view::View;

fn main() {
    let n2 = geometry::P2::new;

    let ts = vec![
        [n2(-1.0, 0.0), n2(-1.0, -1.0), n2(0.0, -1.0)],
        [n2(1.0, 0.0), n2(1.0, 1.0), n2(0.0, 1.0)],
    ];

    let g2 = Geometry2D::from(Triangles(ts));

    let g3 = g2.extrude_linear(1.0);

    let view = View::new("disjointed_extrusion");
    view.run(g3);
}
