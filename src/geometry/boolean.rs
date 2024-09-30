use log::info;

use super::{Geometry2D, P2};


/// Check if 2 line segments intersect
fn is_intersect(a: &[P2; 2], b: &[P2; 2]) -> bool {
    // Equation for a `line` to calculate "y" from `x`
    let f = |line: &[P2; 2], x| line[0].y + (x - line[0].x) * (line[1].y - line[0].y) / (line[1].x - line[0].x);

    // Whether points of `b` are on different sides of the line defined by `a`
    let crosses_a = (f(a, b[0].x) - b[0].y).signum() != (f(a, b[1].x) - b[1].y).signum();
    // Whether points of `a` are on different sides of the line defined by `b`
    let crosses_b = (f(b, a[0].x) - a[0].y).signum() != (f(b, a[1].x) - a[1].y).signum();

    return crosses_a && crosses_b;
}


impl Geometry2D {
    pub fn union(&self, other: &Self) -> Self {
        let edges_a = self.outer_edges();
        let edges_b = other.outer_edges();

        for ea in &edges_a {
            for eb in &edges_b {
                if is_intersect(ea, eb) {
                    info!("Found an intersection:\n -> {:#?}\n -> {:#?}", ea, eb);
                }
            }
        }

        // FIXME
        self.concat(other)
    }
}
