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
            assert_eq!(expected_line, actual_line);
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

    #[test]
    fn canvas_line_split() {
        let mut canvas = Canvas::new(10, 2);

        let c = Color::new(1.0, 0.8, 0.6);

        for y in 0..2 {
            for x in 0..10 {
                canvas.write_pixel(x, y, c).unwrap();
            }
        }

        let expected_lines = "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153";

        let actual = canvas.to_ppm();

        let actual_lines_iter = actual.lines().skip(3);
        for (expected_line, actual_line) in expected_lines.lines().zip(actual_lines_iter) {
            assert_eq!(expected_line, actual_line);
        }
    }

    #[test]
    fn canvas_ppm_ends_with_newline() {
        let canvas = Canvas::new(5, 3);
        let actual = canvas.to_ppm();

        let last_char = actual.chars().last().unwrap();
        assert_eq!(last_char, '\n');
    }
}