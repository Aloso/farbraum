use crate::spaces::{CieXyz, LinearSrgb, Srgb};
use crate::whites::{D50, D65};
use crate::{From, Vec3};

impl From<CieXyz, D50> for Vec3<CieXyz, D65> {
    fn from(xyz: Vec3<CieXyz, D50>) -> Self {
        let (x, y, z) = xyz.tuple();

        let x2 = 0.9555766 * x - 0.0230393 * y + 0.0631636 * z;
        let y2 = -0.0282895 * x + 1.0099416 * y + 0.0210077 * z;
        let z2 = 0.0122982 * x - 0.020483 * y + 1.3299098 * z;

        Vec3::new(x2, y2, z2)
    }
}

impl From<CieXyz, D65> for Vec3<CieXyz, D50> {
    fn from(xyz: Vec3<CieXyz, D65>) -> Self {
        let (x, y, z) = xyz.tuple();

        let x2 = 1.0478112 * x + 0.0228866 * y - 0.050127 * z;
        let y2 = 0.0295424 * x + 0.9904844 * y - 0.0170491 * z;
        let z2 = -0.0092345 * x + 0.0150436 * y + 0.7521316 * z;

        Vec3::new(x2, y2, z2)
    }
}

impl From<LinearSrgb> for Vec3<CieXyz, D65> {
    fn from(rgb: Vec3<LinearSrgb>) -> Self {
        let (r, g, b) = rgb.tuple();

        let x = 0.4124564 * r + 0.3575761 * g + 0.1804375 * b;
        let y = 0.2126729 * r + 0.7151522 * g + 0.072175 * b;
        let z = 0.0193339 * r + 0.119192 * g + 0.9503041 * b;

        Vec3::new(x, y, z)
    }
}

impl From<Srgb> for Vec3<CieXyz, D65> {
    fn from(rgb: Vec3<Srgb>) -> Self {
        rgb.into::<LinearSrgb, _>().into()
    }
}

impl From<CieXyz, D65> for Vec3<LinearSrgb> {
    fn from(xyz: Vec3<CieXyz, D65>) -> Self {
        let (x, y, z) = xyz.tuple();

        let r = x * 3.2404542 - y * 1.5371385 - 0.4985314 * z;
        let g = x * -0.969266 + y * 1.8760108 + 0.041556 * z;
        let b = x * 0.0556434 - y * 0.2040259 + 1.0572252 * z;

        Vec3::new(r, g, b)
    }
}

impl From<CieXyz, D65> for Vec3<Srgb> {
    fn from(xyz: Vec3<CieXyz, D65>) -> Self {
        xyz.into::<LinearSrgb, _>().into()
    }
}
