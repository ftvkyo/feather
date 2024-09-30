use cgmath::Deg;

use feather::*;

use geometry::{Geometry2D, primitives::V2};
use render::view::View;

fn main() {
    let c3 = Geometry2D::circle(3);
    let c3rot = c3.rotate(Deg(45.0).into()).translate(V2::new(-2.0, 1.0));
    let c12 = Geometry2D::circle(12).translate(V2::new(2.0, 0.5));
    let c12scaled = c12.scale(V2::new(0.25, 0.75)).translate(V2::new(0.0, -2.0));

    // Very much not yet boolean
    let all = c3.concat(&c3rot).concat(&c12).concat(&c12scaled);

    let view = View::new("boolean_flat");
    view.run(all.extrude_linear(0.1));
}
