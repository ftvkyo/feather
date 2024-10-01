use log::info;

use super::{Geometry2D, P2};


/// Check if 2 line segments intersect, return the coordinates of the intersection if yes.
fn intersection(a: &[P2; 2], b: &[P2; 2]) -> Option<P2> {
    // FIXME: handle collinear cases

    let slope_a = (a[1].y - a[0].y) / (a[1].x - a[0].x);
    // Equation for the line `a`
    let line_a = |x| a[0].y + (x - a[0].x) * slope_a;

    let slope_b = (b[1].y - b[0].y) / (b[1].x - b[0].x);
    // Equation for the line `b`
    let line_b = |x| b[0].y + (x - b[0].x) * slope_b;

    // Whether points of `b` are on different sides of the line defined by `a`
    let crosses_a = (line_a(b[0].x) - b[0].y).signum() != (line_a(b[1].x) - b[1].y).signum();
    // Whether points of `a` are on different sides of the line defined by `b`
    let crosses_b = (line_b(a[0].x) - a[0].y).signum() != (line_b(a[1].x) - a[1].y).signum();

    if crosses_a && crosses_b {
        let intersection_x = (b[0].y - a[0].y - b[0].x * slope_b + a[0].x * slope_a) / (slope_a - slope_b);
        let intersection_y = line_a(intersection_x);

        Some(P2::new(intersection_x, intersection_y))
    } else {
        None
    }
}


impl Geometry2D {
    pub fn union(&self, other: &Self) -> Self {
        let edges_a = self.outer_edges();
        let edges_b = other.outer_edges();

        for ea in &edges_a {
            for eb in &edges_b {
                if let Some(i) = intersection(ea, eb) {
                    info!("Found an intersection:\n -> {:#?}\n -> {:#?}", ea, eb);
                }
            }
        }

        // FIXME
        self.concat(other)
    }
}
