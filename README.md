# farbraum - Rust crate to convert between color spaces

This crate offers a simple API to convert between various color spaces:

```rs
let hsv = Color::new(120.0, 1.0, 1.0, Hsv);
let lrgb = hsv.into(Srgb).into(LinearSrgb);
```

Most conversion functions are ported from the [culori](https://culorijs.org) javascript library. Some parts were modified to make the results more accurate.

All colors consist of 3 floating-point values.

## Color spaces

Currently supported are

### RGB color spaces

Type          | Description                                                     | Illuminate
--------------|-----------------------------------------------------------------|--------------
`Srgb`        | sRGB, or standard RGB. Often just called "RGB"                  | `D65`
`LinearSrgb`  | Linear sRGB                                                     | `D65`
`Adobe98`     | Adobe RGB 1998                                                  | `D65`

RGB colors consist of the components _red_, _green_ and _blue_. Each component should be in the range [0, 1].

### sRGB-derived color spaces

Type          | Description                                                     | Illuminate
--------------|-----------------------------------------------------------------|--------------
`Hsl`         | HSL representation of sRGB                                      | `D65`
`Hsv`         | HSV representation of sRGB                                      | `D65`
`Hsi`         | HSI representation of sRGB                                      | `D65`
`Hwb`         | HWB representation of sRGB                                      | `D65`

The first component in these colors, _hue_, is always the same. It's a number in the range [0, 360). The other components should be in the range [0, 1].

### Other

Type          | Description                                                     | Illuminate
--------------|-----------------------------------------------------------------|--------------
`CieXyz<I>`   | CIE XYZ (CIE 1931)                                              | generic
`CieLab<I>`   | CIELAB (CIE 1976 L\*a*b)                                        | generic
`CieLch<I>`   | CIELCh (CIE 1976 L\*a*b in cylindrical form)                    | generic
`CieLuv`      | CIELUV (CIE 1976 L\*u*v)                                        | `D50`
`CieLchuv`    | CIELCh<sub>uv</sub> (CIE 1976 L\*u*v in cylindrical form)       | `D50`
`OkLab`       | Oklab                                                           | `D65`
`OkLch`       | Oklab in cylindrical form                                       | `D65`
`DLab`        | DIN 99 L\*a*b                                                   | `D65`
`DLch`        | DIN 99 L\*a*b in cylindrical form                               | `D65`
`Jab`         | J<sub>z</sub>a<sub>z</sub>b<sub>z</sub> (also called JzAzBz)    | `D65` <sup>(1)</sup>
`Jch`         | J<sub>z</sub>a<sub>z</sub>b<sub>z</sub> in cylindrical form     | `D65` <sup>(1)</sup>
`Cubehelix`   | Cubehelix color scheme                                          | `D65` <sup>(1)</sup>

There's no general rule in what ranges the values should be.

`CieXyz`, `CieLab` and `CieLch` can be used with either the D50 or D65 standard illuminate. The illuminate is provided as a generic argument, e.g. `CieXyz<D50>`.

> <sup>(1)</sup> I should verify that.

## Usage

Colors are created with `Color::new()` or `Color::of()`. Use `of` if you want the color space to be inferred:

```rs
let hsv: Color<Hsv> = Color::of(120.0, 1.0, 1.0);
// or
let hsv = Color::new(120.0, 1.0, 1.0, Hsv);
```

To convert one color space to another, use the provided `into` method, which accepts the new color space:

```rs
hsv.into(Srgb).into(CieXyz(D65)).into(Jab)
```

Note that the color spaces are zero-sized types, so they don't exist at runtime. If you want to choose a color space at runtime, you'll need to create an enum such as:

```rs
#[derive(Debug, Clone, Copy)]
enum AnyColorSpace {
    CieXyzD50,
    Srgb,
    Hsl,
    CieLabD50,
    OkLab,
}
```

This can be used instead of the `farbraum`'s builtin color spaces. However, you'll need to implement conversions for this enum to make it useful:

```rs
use farbraum::{
    Into, Color, illuminate::D50,
    spaces::{Srgb, CieXyz, Hsl, CieLab, OkLab},
};

// Convert any color space to sRGB
impl Into<Srgb> for Color<AnyColorSpace> {
    fn into(self, _: Srgb) -> Color<Srgb> {
        let (a, b, c) = self.tuple();
        match self.space() {
            AnyColorSpace::Srgb      => Color::new(a, b, c, Srgb),
            AnyColorSpace::CieXyzD50 => Color::new(a, b, c, CieXyz(D50)).into(),
            AnyColorSpace::Hsl       => Color::new(a, b, c, Hsl).into(),
            AnyColorSpace::CieLabD50 => Color::new(a, b, c, CieLab(D50)).into(),
            AnyColorSpace::OkLab     => Color::new(a, b, c, OkLab).into(),
        }
    }
}
```

## Cargo features

- `double_precision`: Components are floating-point values, by default `f64`. If you disable the `double_precision` feature, `f32` is used instead, which is less precise but faster.
- `serde`: Enable this feature to serialize and deserialize `Color` values.