use super::*;


impl Settings<'static> {

    pub const WINTER : Self = Self {
        name            : Cow::Borrowed("winter"),
        colour_gradient : Gradient { points : Cow::Borrowed(&[
            GradientPoint { x : 0.0   , col : Rgba { r : 0.0  , g : 1.0 , b : 1.0 , a : 1.0 } },
            GradientPoint { x : 0.333 , col : Rgba { r : 0.25 , g : 0.0 , b : 1.0 , a : 1.0 } },
            GradientPoint { x : 0.667 , col : Rgba { r : 0.0  , g : 1.0 , b : 0.0 , a : 1.0 } },
            GradientPoint { x : 1.0   , col : Rgba { r : 0.0  , g : 1.0 , b : 1.0 , a : 1.0 } }
        ]) },
        ..Self::DEFAULT
    };

}
