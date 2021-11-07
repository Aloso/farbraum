use core::fmt;
use std::marker::PhantomData;

use crate::illuminate::D65;

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
        write!(f, "CieLab-{:?}", I::default())
    }
}

/// CIE LCh color space.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct CieLch<I> {
    wp: PhantomData<I>,
}

impl<I: fmt::Debug + Default> fmt::Debug for CieLch<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CieLch-{:?}", I::default())
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
        write!(f, "CieXyz-{:?}", I::default())
    }
}

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
