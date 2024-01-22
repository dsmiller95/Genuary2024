use std::convert::Into;
use std::f64::consts::PI;
use std::ops::Range;
use turtle::{Color, Drawing, Turtle};

const CIRCLE_RADIUS: f64 = 200.0;

fn main() {
    let mut drawing = Drawing::new();
    drawing.set_background_color("#413C58");
    let mut turtle = drawing.add_turtle();
    turtle.set_speed(10);
    turtle.set_pen_size(2.0);
    turtle.use_radians();

    turtle.pen_up();
    turtle.go_to((-CIRCLE_RADIUS, 0.0));
    turtle.pen_down();

    let valid_angles = 5.0..175.0;
    let step = 1.0 / (valid_angles.end - valid_angles.start);
    let mut t = 0.0;
    loop{
        let angle = ping_pong(t, &valid_angles).to_radians();
        let distance_to_circle= CIRCLE_RADIUS * 2.0 * (angle/2.0).sin();

        turtle.right(angle/2.0);
        turtle.forward(distance_to_circle / 10.0);
        turtle.pen_up();
        turtle.forward(distance_to_circle * 8.0 / 10.0);
        turtle.pen_down();
        turtle.forward(distance_to_circle / 10.0);
        turtle.right(angle/2.0);
        turtle.set_pen_color(lerp_color(t * 2.0f64.sqrt()));
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

fn lerp_color(t: f64) -> Color {
    Color::hsl((t % 1.0) * 360.0, 1.0, 0.5)
}
