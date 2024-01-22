use std::convert::Into;
use std::ops::Range;
use rand::Rng;
use turtle::{Color, Drawing};

const DEGREES_PER_TRACE: f64 = 1.0;
const CIRCLE_RADIUS: f64 = 200.0;
const SEGMENT_DISTANCE: f64 = 30.0;

fn main() {
    let palette = generate_palette(DEGREES_PER_TRACE, 5);

    let mut drawing = Drawing::new();
    drawing.set_background_color(palette[0].darken(0.1));
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
    let total_steps = (2.0 / step) as usize;
    for _ in 0..total_steps {
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
        turtle.set_pen_color(lerp_color(t * 3.0f64.sqrt(), &palette[1..]));
        t += step;
    }

    turtle.pen_up();
    loop{
        let offset = -CIRCLE_RADIUS * 2.0;
        turtle.go_to((offset, offset));
        turtle.go_to((offset, offset + -1.0));
    }
}
use rand_chacha::ChaChaRng;
use rand_chacha::rand_core::SeedableRng;
use colourado_iter::{ColorPalette, PaletteType};
fn generate_palette(rand_seed: f64, len: usize) -> Vec<Color> {
    let mut rng = ChaChaRng::seed_from_u64(rand_seed.to_bits());
    rng.gen_range(0.0..1.0);
    rng.gen_range(0.0..1.0);
    rng.gen_range(0.0..1.0);
    rng.gen_range(0.0..1.0);

    let palette = ColorPalette::new(PaletteType::Random, false, &mut rng);

    palette
        .take(len)
        .map(|color| {
            let (r, g, b) = color.to_tuple().into();
            Color{
                red: r as f64 * 255.0,
                green: g as f64* 255.0,
                blue: b as f64* 255.0,
                alpha: 1.0,
            }
        })
        .collect()
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

fn lerp_color(t: f64, pallet_slice: &[Color]) -> Color {

    let t = (t % 1.0) * pallet_slice.len() as f64;
    let color_index_1 = t.floor() as usize;
    let color_index_2 = (color_index_1 + 1) % pallet_slice.len();
    let t = t - t.floor();

    let color_1 = pallet_slice[color_index_1];
    let color_2 = pallet_slice[color_index_2];

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