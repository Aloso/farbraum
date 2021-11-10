# farbraum

Rust crate to convert between color spaces. "Farbraum"
[/ˈfarbraʊ̯m/](http://ipa-reader.xyz/?text=%CB%88farbra%CA%8A%CC%AFm&voice=Marlene)
is German for "color space".

Most conversion functions are ported from the [culori](https://culorijs.org)
javascript library. Some parts were modified to make the results more accurate.

## Usage

Look at [the documentation](https://docs.rs/farbraum).

## Color spaces

Farbraum supports 24 color spaces, including sRGB, HSL, HSV, Oklab, CIELAB,
CIELUV, CIE XYZ and more. [See the full list](./Color_spaces.md).

## Cargo features

- `double-precision`: Components are floating-point values, by default `f64`. If
  you disable the `double-precision` feature, `f32` is used instead.
- `serde`: Enable this feature to serialize and deserialize `Color` values.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or
  <https://opensource.org/licenses/MIT>)

at your option.
