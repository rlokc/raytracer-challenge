use crate::colors::Color;

pub struct Canvas {
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Canvas {
        let mut pixels = Vec::with_capacity(height.try_into().unwrap());
        for _ in 0..height {
            let mut row = Vec::with_capacity(width.try_into().unwrap());
            for _ in 0..width {
                let c = Color::new(0.0, 0.0, 0.0);
                row.push(c);
            }
            pixels.push(row);
        }
        Canvas {pixels}
    }

    pub fn width(&self) -> i32 {
        self.pixels[0].len().try_into().unwrap()
    }

    pub fn height(&self) -> i32 {
        self.pixels.len().try_into().unwrap()
    }

    pub fn write_pixel(&mut self, x: i32, y: i32, color: Color) {
        let y: usize = y.try_into().unwrap();
        let x: usize = x.try_into().unwrap();
        self.pixels[y][x] = color;
    }

    pub fn pixel_at(&self, x: i32, y: i32) -> Color {
        let y: usize = y.try_into().unwrap();
        let x: usize = x.try_into().unwrap();
        self.pixels[y][x]
    }

    pub fn to_ppm(&self) -> String {
        let max_value = 255; 
        let mut res = String::new();
        res.push_str("P3\n");
        res.push_str(&format!("{} {}\n", self.width(), self.height()));
        res.push_str(&format!("{}\n", max_value));

        for row in self.pixels.iter() {
            let row_text = row.iter().fold("".to_string(), |acc, pixel| format!("{acc} {}", pixel.to_ppm(max_value)));
            // Wrap every 70 symbols
            let mut final_row_text = "".to_string();
            let mut line_len = 0;
            for elem in row_text.split(' ') {
                let elem_len = elem.len();
                if elem_len == 0 {
                    continue;
                }
                if line_len + elem_len > 70 {
                    final_row_text.push('\n');
                    line_len = 0;
                }
                final_row_text.push_str(elem);
                final_row_text.push(' ');
                line_len += elem_len;
            }
            let final_row_text = match final_row_text.strip_prefix(' ') {
                Some(s) => s,
                None => &final_row_text,
            };
            res.push_str(&format!("{}\n", final_row_text));
        }
        println!("{res}");

        res
    }
}