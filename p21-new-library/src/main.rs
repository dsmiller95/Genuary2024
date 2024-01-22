use std::convert::Into;
use std::f64::consts::PI;
use std::ops::Range;
use turtle::{Color, Drawing, Turtle};

const DEGREES_PER_TRACE: f64 = PI;
const CIRCLE_RADIUS: f64 = 200.0;
const SEGMENT_DISTANCE: f64 = 30.0;

const PALLETE: [&str; 5] = ["#2e382e","#50c9ce","#72a1e5","#9883e5","#fcd3de"];

fn main() {
    let mut drawing = Drawing::new();
    drawing.set_background_color(PALLETE[0]);
    let mut turtle = drawing.add_turtle();
    turtle.set_speed(25);
    turtle.set_pen_size(2.0);
    turtle.use_radians();

    turtle.pen_up();
    turtle.go_to((-CIRCLE_RADIUS, 0.0));
    turtle.pen_down();

    let valid_angles = 0.0..180.0;
    let step =  DEGREES_PER_TRACE / (valid_angles.end - valid_angles.start);
    let mut t = 0.0;
    loop{
        let angle = ping_pong(t, &valid_angles).to_radians();
        let distance_to_circle= CIRCLE_RADIUS * 2.0 * (angle/1.75).sin();

        turtle.right(angle/2.0);

        let segment_distance = SEGMENT_DISTANCE.min(distance_to_circle/3.0);
        let remaining_distance = distance_to_circle - segment_distance * 3.0;

        turtle.forward(segment_distance);
        turtle.pen_up();
        turtle.forward(remaining_distance/2.0);
        turtle.pen_down();
        turtle.forward(segment_distance);
        turtle.pen_up();
        turtle.forward(remaining_distance/2.0);
        turtle.pen_down();
        turtle.forward(segment_distance);


        turtle.right(angle/2.0);
        turtle.set_pen_color(lerp_color(t * 3.0f64.sqrt(), &PALLETE[1..]));
        t += step;
    }
}

/// Returns a value between min and max, that oscillates back and forth
/// with a period of 1 second
fn ping_pong(t: f64, out_range: &Range<f64>) -> f64 {

    let t = t % 1.0;
    let normalized_res = if t > 0.5 {
        1.0 - t
    } else {
        t
    } * 2.0;

    return normalized_res * (out_range.end - out_range.start) + out_range.start;
}

fn lerp_color(t: f64, pallet_slice: &[&str]) -> Color {

    let t = (t % 1.0) * pallet_slice.len() as f64;
    let color_index_1 = t.floor() as usize;
    let color_index_2 = (color_index_1 + 1) % pallet_slice.len();
    let t = t - t.floor();

    let color_1: Color = pallet_slice[color_index_1].into();
    let color_2: Color = pallet_slice[color_index_2].into();

    color_1.lerp(color_2, t)
}

trait Lerp {
    /// lerp from self towards other, by t. at 0 will return self, at 1 will return other
    fn lerp(&self, other: Self, t: f64) -> Self;
}

impl Lerp for Color {
    fn lerp(&self, other: Self, t: f64) -> Self {
        let t = t.min(1.0).max(0.0);
        let r = self.red.lerp(other.red, t);
        let g = self.green.lerp(other.green, t);
        let b = self.blue.lerp(other.blue, t);
        let a = self.alpha.lerp(other.alpha, t);
        Color::rgba(r, g, b, a)
    }
}

impl Lerp for f64 {
    fn lerp(&self, other: Self, t: f64) -> Self {
        self + (other - self) * t
    }
}