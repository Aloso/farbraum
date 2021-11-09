use crate::Float;

const PI: Float = float!(PI);

pub(crate) fn normalize_hue(hue: Float) -> Float {
    hue.rem_euclid(360.0)
}

pub(crate) fn rad_to_deg(rad: Float) -> Float {
    (rad * 180.0) / PI
}

pub(crate) fn deg_to_rad(rad: Float) -> Float {
    (rad / 180.0) * PI
}

pub(crate) fn min_max(r: Float, g: Float, b: Float) -> (Float, Float) {
    (r.min(g).min(b), r.max(g).max(b))
}

pub(crate) fn no_nan(n: Float) -> Float {
    if n.is_nan() {
        0.0
    } else {
        n
    }
}

/// Calculates the first component (hue) for HSL, HSV, HSI and HWB color spaces
pub(crate) fn calculate_hsl_hue(r: Float, g: Float, b: Float, max: Float, min: Float) -> Float {
    if min != max {
        (if max == r {
            (g - b) / (max - min) + (if g < b { 6.0 } else { 0.0 })
        } else if max == g {
            (b - r) / (max - min) + 2.0
        } else {
            (r - g) / (max - min) + 4.0
        }) * 60.0
    } else {
        0.0
    }
}

pub(crate) fn cos_and_sin_radians(factor: Float, hue: Float) -> (Float, Float) {
    if factor != 0.0 && !factor.is_nan() {
        (
            factor * deg_to_rad(hue).cos(),
            factor * deg_to_rad(hue).sin(),
        )
    } else {
        (0.0, 0.0)
    }
}
