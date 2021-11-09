use core::fmt;
use std::marker::PhantomData;

use crate::illuminate::D65;

mod adobe98;
mod cielab_d50;
mod cielab_d65;
mod cielch;
mod cielchuv;
mod cieluv;
mod cubehelix;
mod dlab;
mod dlch;
mod hsi;
mod hsl;
mod hsv;
mod hwb;
mod jab;
mod jch;
mod lab;
mod lrgb;
mod oklab;
mod oklch;
mod xyz_d50;
mod xyz_d65;

// Missing color spaces from culori:
//     OkHsl, OkHsv, DCI-P3, ProPhoto, Rec.2020
//
//
// CIE:
//     CIECAM97s, CIECAM02, iCAM06, CAM16, YUV-1960, UVW
// RGB:
//     Adobe Wide Gamut RGB, ProPhoto, scRGB, DCI-P3, Rec.709, Rec.2020, Rec.2100
// YUV:
//     YCbCr, YCoCg, YPbPr, YDdDr, YIQ, xvYCC, sYCC, YIQ
// Subtractive color spaces:
//     CMY, CMYK, CcMmYK, RYB
// Other:
//     IPT, ICtCp, XYB, SRLAB2, OSA-UCS, Coloroid, LMS, Hexachrome, Yxy, HCL, polarLAB
//
//
// Color spaces I'd like to add: IPT, ICtCp, Yxy

mod util;

pub trait Lab {
    type Lch;
    type Illuminate;
}

/// Adobe RGB color space from 1998.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Adobe98;

/// (Encoded) sRGB color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Srgb;

/// CIE Lab color space.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct CieLab<I> {
    wp: PhantomData<I>,
}

impl<I: fmt::Debug + Default> fmt::Debug for CieLab<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CieLab<{:?}>", I::default())
    }
}

/// CIE LCh color space.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct CieLch<I> {
    wp: PhantomData<I>,
}

impl<I: fmt::Debug + Default> fmt::Debug for CieLch<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CieLch<{:?}>", I::default())
    }
}

impl<I> Lab for CieLab<I> {
    type Lch = CieLch<I>;
    type Illuminate = I;
}

/// CIE XYZ color space.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct CieXyz<I> {
    illuminate: PhantomData<I>,
}

impl<I: fmt::Debug + Default> fmt::Debug for CieXyz<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CieXyz<{:?}>", I::default())
    }
}

/// Cie LCh color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CieLuv;

/// Cie LChuv color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CieLchuv;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CubeHelix;

/// DIN99 Lab color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DLab;

/// DIN99 LCh color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DLch;

impl Lab for DLab {
    type Lch = DLch;
    type Illuminate = D65;
}

/// Jab color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Jab;

/// Jch color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Jch;

/// Ok Lab color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OkLab;

/// Ok LCh color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OkLch;

impl Lab for OkLab {
    type Lch = OkLch;
    type Illuminate = D65;
}

/// HSI color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Hsi;

/// HSL color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Hsl;

/// HSV color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Hsv;

/// HWB color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Hwb;

/// Linear sRGB color space.
///
/// This is usually not used directly, only as intermediate value
/// when converting between a XYZ or Lab color space and (encoded) sRGB.
///
/// When people speak about (s)RGB, they _usually_ mean encoded sRGB,
/// not linear sRGB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LinearSrgb;
