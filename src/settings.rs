use crate::util::*;
use std::borrow::Cow;
use std::f32::consts::FRAC_PI_4;


mod default;
mod winter;
mod holiday;
mod autumn;
mod halloween;


pub struct Settings<'l> {
    pub name                   : Cow<'static, str>,
    pub colour_rotate          : f32,
    pub centre_colour          : Rgba,
    pub voronoi_scale          : f32,
    pub voronoi_amount         : f32,
    pub shape_sides            : usize,
    pub shape_radius           : f32,
    pub shape_dropoff_width    : f32,
    pub shape_dropoff_rotate   : f32,
    pub colour_gradient        : Gradient<'l>,
    pub centre_curve           : Curve<'l>,
    pub transparency_curve     : Curve<'l>,
    pub shape_dropoff_gradient : Gradient<'l>
}
