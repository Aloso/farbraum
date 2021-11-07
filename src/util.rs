use crate::Float;

pub(crate) fn normalize_hue(hue: Float) -> Float {
    hue.rem_euclid(360.0)
}

pub(crate) fn min_max(r: Float, g: Float, b: Float) -> (Float, Float) {
    (r.max(g).max(b), r.min(g).min(b))
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
