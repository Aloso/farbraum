#![macro_use]

use crate::{spaces::Srgb, Color, From};

#[macro_export]
macro_rules! assert_similar {
    ($e1:expr, $e2:expr, finally: $finally:expr) => {{
        let _e1 = $e1;
        let _e2 = $e2;
        let (_v0, _v1, _v2) = _e1.tuple();
        let (_v3, _v4, _v5) = _e2.tuple();

        let _equal0 = (_v0 - _v3).abs() < 0.000002;
        let _equal1 = (_v1 - _v4).abs() < 0.000002;
        let _equal2 = (_v2 - _v5).abs() < 0.000002;
        if !_equal0 || !_equal1 || !_equal2 {
            $finally;
            eprintln!(
                "  Values are sufficiently similar at position 1:{}, 2:{}, 3:{}\n",
                _equal0, _equal1, _equal2
            );
            assert_eq!(_e1, _e2);
        }
    }};
    ($e1:expr, $e2:expr) => {
        assert_similar!($e1, $e2, finally: ())
    };
}

fn get_colors() -> Vec<Color<Srgb>> {
    vec![
        Color::<Srgb>::new(0.0, 0.0, 0.0),
        Color::<Srgb>::new(1.0, 1.0, 1.0),
        Color::<Srgb>::new(1.0, 0.0, 0.0),
        Color::<Srgb>::new(0.4, 0.1, 0.0),
        Color::<Srgb>::new(1.0, 0.1, 0.0),
        Color::<Srgb>::new(0.4, 1.0, 0.0),
        Color::<Srgb>::new(0.4, 1.0, 1.0),
        Color::<Srgb>::new(0.9, 0.2, 1.0),
        Color::<Srgb>::new(0.5, 0.5, 0.5),
        Color::<Srgb>::new(0.5, 0.5, 0.55),
        Color::<Srgb>::new(0.001, 0.001, 0.001),
        Color::<Srgb>::new(0.617, 0.367, 0.964),
    ]
}

#[track_caller]
pub fn round_trips_srgb<T>()
where
    Color<T>: From<Srgb>,
    Color<Srgb>: From<T>,
{
    for color in get_colors() {
        let converted = color.into::<T>().into::<Srgb>();
        assert_similar!(
            color,
            converted,
            finally: println!("--> color: {:?}", color)
        );
    }
}

#[track_caller]
pub fn round_trips<T: Copy, U>()
where
    Color<T>: From<U>,
    Color<U>: From<T>,
    Color<T>: From<Srgb>,
    Color<Srgb>: From<T>,
    T: core::fmt::Debug + Default,
{
    for color in get_colors() {
        let init = color.into::<T>();
        let converted = init.into::<U>().into::<T>();
        assert_similar!(init, converted, finally: {
            println!("--> color: {:?}", color);
            println!("--> init: {:?}", init);
        });
        assert_similar!(color, converted.into(), finally: {
            println!("--> color: {:?}", color);
        });
    }
}
