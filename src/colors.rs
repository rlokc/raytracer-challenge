use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub color_tuple: Tuple,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        let t = Tuple::point(red, green, blue);
        Color {color_tuple: t}
    }

    pub fn red(&self) -> f32 {
        self.color_tuple.x
    }

    pub fn green(&self) -> f32 {
        self.color_tuple.y
    }

    pub fn blue(&self) -> f32 {
        self.color_tuple.z
    }

    pub fn equals(&self, other: Color) -> bool {
        self.color_tuple.equals(other.color_tuple)
    }

    pub fn add(&self, other: Color) -> Color {
        let c = Color {color_tuple: self.color_tuple.add(other.color_tuple)};
        c.reset_point()
    }

    pub fn sub(&self, other: Color) -> Color {
        let c = Color {color_tuple: self.color_tuple.sub(other.color_tuple)};
        c.reset_point()
    }

    pub fn scalar_mul(&self, a: f32) -> Color {
        let c = Color {color_tuple: self.color_tuple.scalar_mul(a)};
        c.reset_point()
    }

    pub fn mul(&self, other: Color) -> Color {
        Color::new(self.red() * other.red(), self.green() * other.green(), self.blue() * other.blue())
    }

    fn reset_point(mut self) -> Color {
        self.color_tuple.w = 1.0;
        self
    }
}