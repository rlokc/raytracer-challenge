#[cfg(test)]
mod canvas_tests {
    use raytracer::{colors::Color, canvas::Canvas};

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

    #[test]
    fn canvas_create() {
        let canvas = Canvas::new(10, 20);

        let black = Color::new(0.0, 0.0, 0.0);

        assert_eq!(canvas.width(), 10);
        assert_eq!(canvas.height(), 20);

        for row in canvas.pixels.iter() {
            for pixel in row.iter() {
                assert!(pixel.equals(black));
            }
        }
    }

    #[test]
    fn canvas_test_write() {
        let mut canvas = Canvas::new(10, 20);
        let red =  Color::new(1.0, 0.0, 0.0);

        canvas.write_pixel(2, 3, red);
        
        let actual = canvas.pixel_at(2, 3);
        assert!(actual.equals(red));
    }

    #[test]
    fn canvas_ppm_header() {
        let canvas = Canvas::new(5, 3);
        let expected = "P3\n5 3\n255";

        let actual = canvas.to_ppm();

        for (expected_line, actual_line) in expected.lines().take(3).zip(actual.lines()) {
            match (expected_line, actual_line) {
                (l1, l2) => {
                    assert_eq!(l1, l2);
                }
            }
        }
    }

    #[test]
    fn canvas_ppm() {
        let mut canvas = Canvas::new(5, 3);

        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);


        canvas.to_ppm();
    }

}