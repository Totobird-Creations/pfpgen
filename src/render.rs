use crate::util::*;
use crate::Settings;
use core::f32::consts::{ TAU, PI, FRAC_PI_2 };
use rand::SeedableRng;
use rand::distr::{ Distribution, StandardUniform };
use rand_chacha::ChaChaRng;


fn uv_seed(uv : F32x2) -> u64 {
    ((uv.x.to_bits() as u64) << 32) | (uv.y.to_bits() as u64)
}


fn voronoi(uv : F32x2, cells : F32x2) -> f32 {
    let uv_cells = uv * cells;
    let index_uv = uv_cells.floor();
    let fract_uv = uv_cells.fract();
    let mut min_dist  = 1.0;
    let mut min_point = F32x2::ZERO;
    for y in (-1)..=1 {
        for x in (-1)..=1 {
            let     neighbor  = F32x2 { x : x as f32, y : y as f32 };
            let     point_arg = index_uv + neighbor;
            let mut rng       = ChaChaRng::seed_from_u64(uv_seed(point_arg));
            let     point     = Distribution::<f32>::sample(&StandardUniform, &mut rng);
            let     point     = F32x2 { x : point, y : point };
            let     diff      = neighbor + point - fract_uv;
            let     dist      = diff.len();
            if (dist < min_dist) {
                min_dist  = dist;
                min_point = point;
            }
        }
    }
    min_point.x
}


pub fn shape(uv : F32x2, sides : usize, radius : f32, edge : f32, edge_gradient : &Gradient<'_>, edge_gradient_rotate : f32) -> Rgba {
    let sides        = sides as f32;
    let centre       = F32x2::CENTRE;
    let dist         = uv.dist(centre) * 2.0;
    let angle        = ((uv - centre).atan2() + FRAC_PI_2).rem_euclid(TAU) + (PI / sides);
    let radius       = radius * (PI / sides).cos() / (angle - (TAU / sides) * ((sides * angle + PI) / TAU).floor()).cos();
    let inner_radius = radius * edge;
    if (dist <= inner_radius) {
        edge_gradient.sample(0.125) + voronoi(uv, F32x2 { x : 12.5, y : 12.5 }) * 0.05
    } else if (dist <= radius) {
        let section = ((angle + (PI / sides)) / TAU * sides) as usize;
        let x       = ((section as f32) / sides - edge_gradient_rotate).rem_euclid(1.0);
        edge_gradient.sample(x)
    } else {
        Rgba::TRANSPARENT
    }
}


pub fn fragment(settings : &Settings, uv : F32x2) -> Rgba {

    // Alpha
    let alpha = settings.transparency_curve.sample(uv.dist(F32x2::CENTRE) * 2.0);

    // Rainbow Colour
    let     colour = settings.colour_gradient.sample(((uv - F32x2::CENTRE).atan2() / TAU + 0.25 - settings.colour_rotate).rem_euclid(1.0));
    let mut out    = colour;

    // Voronoi pattern
    let voronoi = (voronoi(uv, F32x2 { x : settings.voronoi_scale, y : settings.voronoi_scale }) * 2.0 - 1.0) * settings.voronoi_amount;
    out         = Rgba { r : out.r + voronoi, g : out.g + voronoi, b : out.b + voronoi, a : out.a * alpha };

    // Centre Gradient
    let centre = settings.centre_curve.sample(uv.dist(F32x2::CENTRE) * 2.0);
    let alpha  = out.a;
    out        = out.mix(colour.mix(settings.centre_colour, 0.625), centre * settings.centre_colour.a);
    out.a      = alpha;

    // Shape
    let shape = shape(
        uv,
        settings.shape_sides,
        settings.shape_radius,
        settings.shape_dropoff_width,
        &settings.shape_dropoff_gradient,
        settings.shape_dropoff_rotate
    );
    let alpha = out.a;
    out       = out.mix(shape, shape.a);
    out.a     = alpha;

    out
    // Rgba { r : out.r.clamp(0.0, 1.0), g : out.g.clamp(0.0, 1.0), b : out.b.clamp(0.0, 1.0), a : out.a.clamp(0.0, 1.0) }
}
