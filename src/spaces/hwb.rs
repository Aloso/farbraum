use crate::spaces::{Hsv, Hwb, Srgb};
use crate::{From, Vec3};

impl From<Hwb> for Vec3<Hsv> {
    fn from(hwb: Vec3<Hwb>) -> Self {
        let (h, mut w, mut b) = hwb.tuple();

        // normalize w + b to 1
        if w + b > 1.0 {
            let s = w + b;
            w /= s;
            b /= s;
        }
        Vec3::new(h, if b == 1.0 { 1.0 } else { 1.0 - w / (1.0 - b) }, 1.0 - b)
    }
}

impl From<Hwb> for Vec3<Srgb> {
    fn from(hwb: Vec3<Hwb>) -> Self {
        hwb.into::<Hsv, _>().into()
    }
}

impl From<Hsv> for Vec3<Hwb> {
    fn from(hsv: Vec3<Hsv>) -> Self {
        let (h, s, v) = hsv.tuple();

        let w = (1.0 - s) * v;
        let b = 1.0 - v;

        Vec3::new(h, w, b)
    }
}

impl From<Srgb> for Vec3<Hwb> {
    fn from(rgb: Vec3<Srgb>) -> Self {
        rgb.into::<Hsv, _>().into()
    }
}
