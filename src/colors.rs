use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub color_tuple: Tuple,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        return self.color_tuple == other.color_tuple;
    }
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        let t = Tuple::point(red, green, blue);
        Color { color_tuple: t }
    }

    pub fn red(&self) -> f64 {
        self.color_tuple.x
    }

    pub fn green(&self) -> f64 {
        self.color_tuple.y
    }

    pub fn blue(&self) -> f64 {
        self.color_tuple.z
    }

    pub fn equals(&self, other: Color) -> bool {
        self.color_tuple == other.color_tuple
    }

    pub fn add(&self, other: Color) -> Color {
        let c = Color {
            color_tuple: self.color_tuple.add(other.color_tuple),
        };
        c.reset_point()
    }

    pub fn sub(&self, other: Color) -> Color {
        let c = Color {
            color_tuple: self.color_tuple.sub(other.color_tuple),
        };
        c.reset_point()
    }

    pub fn scalar_mul(&self, a: f64) -> Color {
        let c = Color {
            color_tuple: self.color_tuple.scalar_mul(a),
        };
        c.reset_point()
    }

    pub fn mul(&self, other: Color) -> Color {
        Color::new(
            self.red() * other.red(),
            self.green() * other.green(),
            self.blue() * other.blue(),
        )
    }

    fn reset_point(mut self) -> Color {
        self.color_tuple.w = 1.0;
        self
    }

    pub fn to_ppm(&self, max_value: i32) -> String {
        let max_value = max_value as f64;
        let red = (max_value * self.red().clamp(0.0, 1.0)).round() as i32;
        let green = (max_value * self.green().clamp(0.0, 1.0)).round() as i32;
        let blue = (max_value * self.blue().clamp(0.0, 1.0)).round() as i32;
        format!("{} {} {}", red, green, blue)
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        let max_value = 255f64;
        let red = (max_value * self.red().clamp(0.0, 1.0)).round() as u8;
        let green = (max_value * self.green().clamp(0.0, 1.0)).round() as u8;
        let blue = (max_value * self.blue().clamp(0.0, 1.0)).round() as u8;
        [red, green, blue]
    }
}
