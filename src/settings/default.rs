use super::*;


impl Settings<'static> {

    pub const DEFAULT : Self = Self {
        name                   : Cow::Borrowed("default"),
        colour_rotate          : 0.0,
        centre_colour          : Rgba { r : 1.0625, g : 1.0625, b : 1.0625, a : 1.0 },
        voronoi_scale          : 15.0,
        voronoi_amount         : 0.375,
        shape_sides            : 6,
        shape_radius           : 0.563,
        shape_dropoff_width    : 0.825,
        shape_dropoff_rotate   : 0.0,
        colour_gradient        : Gradient { points : Cow::Borrowed(&[
            GradientPoint { x : 0.0   , col : Rgba { r : 1.0, g : 0.0, b : 0.0, a : 1.0 } },
            GradientPoint { x : 0.167 , col : Rgba { r : 1.0, g : 1.0, b : 0.0, a : 1.0 } },
            GradientPoint { x : 0.333 , col : Rgba { r : 0.0, g : 1.0, b : 0.0, a : 1.0 } },
            GradientPoint { x : 0.5   , col : Rgba { r : 0.0, g : 1.0, b : 1.0, a : 1.0 } },
            GradientPoint { x : 0.667 , col : Rgba { r : 0.0, g : 0.0, b : 1.0, a : 1.0 } },
            GradientPoint { x : 0.833 , col : Rgba { r : 1.0, g : 0.0, b : 1.0, a : 1.0 } },
            GradientPoint { x : 1.0   , col : Rgba { r : 1.0, g : 0.0, b : 0.0, a : 1.0 } }
        ]) },
        centre_curve           : Curve { points : Cow::Borrowed(&[
            CurvePoint { pos : F32x2 { x : 0.0,       y : 1.0      }, ltan :  0.0       , rtan :  0.122728  },
            CurvePoint { pos : F32x2 { x : 0.366667 , y : 1.0      }, ltan :  0.0       , rtan :  0.0       },
            CurvePoint { pos : F32x2 { x : 0.588889 , y : 0.763636 }, ltan : -2.98386   , rtan : -2.98386   },
            CurvePoint { pos : F32x2 { x : 0.805556 , y : 0.0      }, ltan : -0.0761099 , rtan : -0.0761099 },
            CurvePoint { pos : F32x2 { x : 1.0      , y : 0.0      }, ltan :  0.0743801 , rtan :  0.0       }
        ]) },
        transparency_curve     : Curve { points : Cow::Borrowed(&[
            CurvePoint { pos : F32x2 { x : 0.0 , y : 1.0 }, ltan : 0.0 , rtan : 0.0 }
        ]) },
        shape_dropoff_gradient : Gradient { points : Cow::Borrowed(&[
            GradientPoint { x : 0.0, col : Rgba { r : 0.9, g : 0.9, b : 0.9, a : 1.0 } },
            GradientPoint { x : 0.5, col : Rgba { r : 0.5, g : 0.5, b : 0.5, a : 1.0 } },
            GradientPoint { x : 1.0, col : Rgba { r : 0.9, g : 0.9, b : 0.9, a : 1.0 } }
        ]) }
    };

}
