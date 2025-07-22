use super::*;


impl Settings<'static> {

    pub const HALLOWEEN : Self = Self {
        name            : Cow::Borrowed("halloween"),
        colour_rotate   : FRAC_PI_4,
        colour_gradient : Gradient { points : Cow::Borrowed(&[
            // GradientPoint { x : 0.0 , col : Rgba { r : 0.0 , g : 1.0 , b : 0.0 , a : 1.0 } },
            // GradientPoint { x : 0.5 , col : Rgba { r : 0.5 , g : 0.0 , b : 1.0 , a : 1.0 } },
            // GradientPoint { x : 1.0 , col : Rgba { r : 0.0 , g : 1.0 , b : 0.0 , a : 1.0 } }

            GradientPoint { x : 0.0  , col : Rgba { r : 0.0  , g : 1.0 , b : 0.0 , a : 1.0 } },
            GradientPoint { x : 0.25 , col : Rgba { r : 0.25 , g : 1.0 , b : 0.0 , a : 1.0 } },
            GradientPoint { x : 0.5  , col : Rgba { r : 0.75 , g : 0.0 , b : 1.0 , a : 1.0 } },
            GradientPoint { x : 0.75 , col : Rgba { r : 0.5  , g : 0.0 , b : 1.0 , a : 1.0 } },
            GradientPoint { x : 1.0  , col : Rgba { r : 0.0  , g : 1.0 , b : 0.0 , a : 1.0 } }
        ]) },
        ..Self::DEFAULT
    };

}
