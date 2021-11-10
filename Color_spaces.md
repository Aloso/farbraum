# Color spaces supported by `farbraum`

All color spaces use the
[D65 standard illuminate](https://en.wikipedia.org/wiki/Illuminant_D65) unless
stated otherwise.

## RGB color spaces

| Type         | Description                                    |
| ------------ | ---------------------------------------------- |
| `Srgb`       | sRGB, or standard RGB. Often just called "RGB" |
| `LinearSrgb` | Linear sRGB                                    |
| `Adobe98`    | Adobe RGB 1998                                 |

RGB colors consist of the components _red_, _green_ and _blue_. Each component
should be in the range [0, 1].

## CMY (subtractive) color spaces

| Type   | Description                                                                                                                    |
| ------ | ------------------------------------------------------------------------------------------------------------------------------ |
| `Cmy`  | CMY, inverse of sRGB                                                                                                           |
| `Cmyk` | CMYK, the color space used by color printers<br>Represented as a tuple instead of a `Color` struct because it has 4 components |

CMY consists of the components _cyan_, _magenta_ and _yellow_. Each component
should be in the range [0, 1].

## sRGB-derived color spaces

| Type  | Description                |
| ----- | -------------------------- |
| `Hsl` | HSL representation of sRGB |
| `Hsv` | HSV representation of sRGB |
| `Hsi` | HSI representation of sRGB |
| `Hwb` | HWB representation of sRGB |

The first component in these colors, _hue_, is always the same. It's a number in
the range [0, 360). The other components should be in the range [0, 1].

## Other

| Type        | Description                                                  | Illuminate |
| ----------- | ------------------------------------------------------------ | ---------- |
| `CieXyz<I>` | CIE XYZ (CIE 1931)                                           | generic    |
| `CieLab<I>` | CIELAB (CIE 1976 L\*a*b)                                     | generic    |
| `CieLch<I>` | CIELCh (CIE 1976 L\*a*b in cylindrical form)                 | generic    |
| `CieLuv`    | CIELUV (CIE 1976 L\*u*v)                                     | `D50`      |
| `CieLchuv`  | CIELCh<sub>uv</sub> (CIE 1976 L\*u*v in cylindrical form)    | `D50`      |
| `OkLab`     | Oklab                                                        |            |
| `OkLch`     | Oklab in cylindrical form                                    |            |
| `DLab`      | DIN 99 L\*a*b                                                |            |
| `DLch`      | DIN 99 L\*a*b in cylindrical form                            |            |
| `Jab`       | J<sub>z</sub>a<sub>z</sub>b<sub>z</sub> (also called JzAzBz) |            |
| `Jch`       | J<sub>z</sub>a<sub>z</sub>b<sub>z</sub> in cylindrical form  |            |
| `Cubehelix` | Cubehelix color scheme                                       |            |

There's no general rule in what ranges the values should be.

`CieXyz`, `CieLab` and `CieLch` can be used with either the D50 or D65 standard
illuminate. The illuminate is provided as a generic argument, e.g.
`CieXyz<D50>`.
