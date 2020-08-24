use num::Float;

struct Rgb<F: Float> {
    r: F,
    g: F,
    b: F,
}

fn rescale<F: Float>(float: F) -> u8 {
    num::cast((float * F::from(255).unwrap()).round()).unwrap()
}

impl<F: Float> Into<image::Rgb<u8>> for Rgb<F> {
    fn into(self) -> image::Rgb<u8> {
        image::Rgb::from([rescale(self.r), rescale(self.g), rescale(self.b)])
    }
}

#[cfg(test)]
mod tests {
    use super::rescale;

    #[test]
    fn rescale_examples() {
        assert_eq!(rescale(0f32), 0);
        assert_eq!(rescale(0f64), 0);
        assert_eq!(rescale(1f32), 255);
        assert_eq!(rescale(1f64), 255);
        assert_eq!(rescale(0.5f32), 128);
        assert_eq!(rescale(0.5f64), 128);
    }
}
