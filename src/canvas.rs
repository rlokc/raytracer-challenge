use std::{fs::File, io::Write};
use std::io::BufWriter;

use crate::{colors::Color, utils::remove_suffix};

#[derive(Debug)]
pub struct Canvas {
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
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

    pub fn width(&self) -> usize {
        self.pixels[0].len().try_into().unwrap()
    }

    pub fn height(&self) -> usize {
        self.pixels.len().try_into().unwrap()
    }

    // Writes a pixel and returns the written color
    // If the pixel was out of bounds returns None
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) -> Option<Color> {
        if !(0..self.height()).contains(&y) {
            return None;
        }
        if !(0..self.width()).contains(&x) {
            return None
        }
        let y: usize = y.try_into().unwrap();
        let x: usize = x.try_into().unwrap();
        self.pixels[y][x] = color;
        Some(color)
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
                    final_row_text = remove_suffix(final_row_text, " ");
                    final_row_text.push('\n');
                    line_len = 0;
                }
                final_row_text.push_str(elem);
                final_row_text.push(' ');
                line_len += elem_len + 1;
            }
            final_row_text = remove_suffix(final_row_text, " ");
            res.push_str(&format!("{}\n", final_row_text));
        }
        res
    }

    pub fn to_png_file(&self, path: &str) {
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width() as u32, self.height() as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        // encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));
        let mut writer = encoder.write_header().unwrap();

        let mut stream_writer = writer.stream_writer().unwrap();
        for row in self.pixels.iter() {
            for pixel in row.iter() {
                stream_writer.write_all(&pixel.to_rgb()).unwrap();
            }
        }
        println!("Writing PNG to {}", path);
        stream_writer.finish().unwrap();
    }

    pub fn to_ppm_file(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        println!("Writing PPM to {}", path);
        file.write_all(self.to_ppm().as_bytes())
    }
}