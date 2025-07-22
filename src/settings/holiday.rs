use super::*;


impl Settings<'static> {

    pub const HOLIDAY : Self = Self {
        name            : Cow::Borrowed("holiday"),
        colour_gradient : Gradient { points : Cow::Borrowed(&[
            GradientPoint { x : 0.0   , col : Rgba { r : 1.0 , g : 0.0 , b : 0.0 , a : 1.0 } },
            GradientPoint { x : 0.125 , col : Rgba { r : 1.0 , g : 1.0 , b : 0.0 , a : 1.0 } },
            GradientPoint { x : 0.25  , col : Rgba { r : 0.0 , g : 1.0 , b : 0.0 , a : 1.0 } },
            GradientPoint { x : 0.375 , col : Rgba { r : 1.0 , g : 1.0 , b : 0.0 , a : 1.0 } },
            GradientPoint { x : 0.5   , col : Rgba { r : 1.0 , g : 0.0 , b : 0.0 , a : 1.0 } },
            GradientPoint { x : 0.625 , col : Rgba { r : 1.0 , g : 1.0 , b : 0.0 , a : 1.0 } },
            GradientPoint { x : 0.75  , col : Rgba { r : 0.0 , g : 1.0 , b : 0.0 , a : 1.0 } },
            GradientPoint { x : 0.875 , col : Rgba { r : 1.0 , g : 1.0 , b : 0.0 , a : 1.0 } },
            GradientPoint { x : 1.0   , col : Rgba { r : 1.0 , g : 0.0 , b : 0.0 , a : 1.0 } }
        ]) },
        ..Self::DEFAULT
    };

}
