mod adobe98;
mod cielab_d50;
mod cielab_d65;
mod cielch;
mod cubehelix;
mod dlab;
mod dlch;
mod hsi;
mod hsl;
mod hsv;
mod hwb;
mod jab;
mod jch;
mod lrgb;
mod xyz_d50;
mod xyz_d65;

pub trait Lab {
    type Lch;
}

/// Adobe RGB color space from 1998.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Adobe98;

/// (Encoded) sRGB color space.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Srgb;

/// CIE Lab color space.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CieLab;

/// CIE LCh color space.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CieLch;

impl Lab for CieLab {
    type Lch = CieLch;
}

/// CIE XYZ color space.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CieXyz;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CubeHelix;

/// DLab color space.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DLab;

/// DLCh color space.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DLch;

impl Lab for DLab {
    type Lch = DLch;
}

/// HSI color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hsi;

/// HSL color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hsl;

/// HSV color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hsv;

/// HWB color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hwb;

/// Linear sRGB color space.
///
/// This is usually not used directly, only as intermediate value
/// when converting between a XYZ or Lab color space and (encoded) sRGB.
///
/// When people speak about (s)RGB, they _usually_ mean encoded sRGB,
/// not linear sRGB.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LinearSrgb;
