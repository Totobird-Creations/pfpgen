use super::*;


impl Settings<'static> {

    pub const AUTUMN : Self = Self {
        name            : Cow::Borrowed("autumn"),
        colour_gradient : Gradient { points : Cow::Borrowed(&[
            GradientPoint { x : 0.0   , col : Rgba { r : 1.0 , g : 0.5  , b : 1.0 , a : 1.0 } },
            GradientPoint { x : 0.333 , col : Rgba { r : 1.0 , g : 0.75 , b : 0.0 , a : 1.0 } },
            GradientPoint { x : 0.667 , col : Rgba { r : 1.0 , g : 0.0  , b : 0.0 , a : 1.0 } },
            GradientPoint { x : 1.0   , col : Rgba { r : 1.0 , g : 0.5  , b : 1.0 , a : 1.0 } }
        ]) },
        ..Self::DEFAULT
    };

}
