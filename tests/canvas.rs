#[cfg(test)]
mod canvas_tests {
    use raytracer::{colors::Color};

    #[test]
    fn color_test() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red(), -0.5);
        assert_eq!(c.green(), 0.4);
        assert_eq!(c.blue(), 1.7);
    }

    #[test]
    fn color_add() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let expected = Color::new(1.6, 0.7, 1.0);
        let actual = c1.add(c2);
        assert!(expected.equals(actual));
    }

    #[test]
    fn color_sub() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let expected = Color::new(0.2, 0.5, 0.5);
        let actual = c1.sub(c2);
        assert!(expected.equals(actual));
    }

    #[test]
    fn color_scalar_mult() {
        let c1 = Color::new(0.2, 0.3, 0.4);

        let expected = Color::new(0.4, 0.6, 0.8);
        let actual = c1.scalar_mul(2.0);
        assert!(expected.equals(actual));
    }

    #[test]
    fn color_mult() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        let expected = Color::new(0.9, 0.2, 0.04);
        let actual = c1.mul(c2);
        assert!(expected.equals(actual));
    }

}