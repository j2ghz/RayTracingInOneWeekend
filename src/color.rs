use num::Float;

#[derive(Clone, Copy, Debug)]
pub struct Rgb<F: Float> {
    r: F,
    g: F,
    b: F,
}

impl<F: Float> Rgb<F> {
    pub fn new(r: F, g: F, b: F) -> Rgb<F> {
        debug_assert!(r >= F::zero());
        debug_assert!(r <= F::one());
        debug_assert!(g >= F::zero());
        debug_assert!(g <= F::one());
        debug_assert!(b >= F::zero());
        debug_assert!(b <= F::one());
        Rgb { r, g, b }
    }

    pub fn as_color(self) -> image::Rgb<u8> {
        image::Rgb::from([rescale(self.r), rescale(self.g), rescale(self.b)])
    }
}

fn rescale<F: Float>(float: F) -> u8 {
    num::cast((float * F::from(255).unwrap()).round()).unwrap()
}

impl<F: Float> Into<image::Rgb<u8>> for Rgb<F> {
    fn into(self) -> image::Rgb<u8> {
        self.as_color()
    }
}

#[cfg(test)]
mod tests {
    use super::{rescale, Rgb};

    #[test]
    fn rescale_examples() {
        assert_eq!(rescale(0f32), 0);
        assert_eq!(rescale(0f64), 0);
        assert_eq!(rescale(1f32), 255);
        assert_eq!(rescale(1f64), 255);
        assert_eq!(rescale(0.5f32), 128);
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
