#[derive(Clone, Copy, Debug)]
pub struct Rgb {
    r: f64,
    g: f64,
    b: f64,
}

impl Rgb {
    pub fn new(r: f64, g: f64, b: f64) -> Rgb {
        debug_assert!(r >= 0.0);
        debug_assert!(r <= 1.0);
        debug_assert!(g >= 0.0);
        debug_assert!(g <= 1.0);
        debug_assert!(b >= 0.0,"b: {}",b);
        debug_assert!(b <= 1.0);
        Rgb { r, g, b }
    }

    pub fn as_color(self) -> image::Rgb<u8> {
        image::Rgb::from([rescale(self.r), rescale(self.g), rescale(self.b)])
    }
}

fn rescale(float: f64) -> u8 {
    (float * 255.0).round() as u8
}

impl Into<image::Rgb<u8>> for Rgb {
    fn into(self) -> image::Rgb<u8> {
        self.as_color()
    }
}

#[cfg(test)]
mod tests {
    use super::{rescale, Rgb};

    #[test]
    fn rescale_examples() {
        assert_eq!(rescale(0f64), 0);
        assert_eq!(rescale(1f64), 255);
        assert_eq!(rescale(0.5f64), 128);
    }

    #[test]
    fn color_000000() {
        assert_eq!(Rgb::new(0f64, 0f64, 0f64).as_color().0, [0, 0, 0]);
    }
    #[test]
    fn color_fffffff() {
        assert_eq!(Rgb::new(1f64, 1f64, 1f64).as_color().0, [255, 255, 255]);
    }
    #[test]
    fn color_ff0000() {
        assert_eq!(Rgb::new(1f64, 0f64, 0f64).as_color().0, [255, 0, 0]);
    }
}
