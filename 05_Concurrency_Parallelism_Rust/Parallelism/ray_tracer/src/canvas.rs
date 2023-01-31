use crate::tuple::Color;

pub struct Canvas {
    width: i32,
    pub height: i32,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Self {
        let capacity = width * height;
        let mut pixels = Vec::with_capacity(capacity as usize);
        for _ in 0..capacity {
            pixels.push(Color::new(0.0, 0.0, 0.0));
        }

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn write_pixel(&mut self, x: i32, y: i32, color: Color) {
        let idx = self.point_to_index(x, y);
        if idx < self.pixels.len() {
            self.pixels[idx] = color;
        }
    }

    fn pixel_at(&self, x: i32, y: i32) -> Option<&Color> {
        let idx = self.point_to_index(x, y);
        self.pixels.get(idx)
    }

    fn point_to_index(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    // Color values are scaled bewteen 0 and 255: 0:0-1:255
    // This algorithm runs pretty slow.
    // At 500x300 canvas: "cargo run  7.40s user 4.33s system 99% cpu 11.822 total"
    pub fn to_ppm(&self) -> String {
        const MAX_VALUE: i32 = 255;
        let mut ppm = format!("P3\n{} {}\n{}\n", self.width, self.height, MAX_VALUE);
        let scaled_pixels: Vec<[i32; 3]> = self
            .pixels
            .iter()
            .map(|color| scale_color(color, MAX_VALUE))
            .collect();

        const LINE_SIZE: i32 = 70;
        for chunk in scaled_pixels.chunks(self.width as usize) {
            let mut char_count = 0;
            let color_values = chunk.iter().flatten().map(|values| values.to_string());
            for value in color_values {
                let next_char_count = char_count + value.len() as i32 + 1; // for the space
                if next_char_count > LINE_SIZE {
                    ppm.pop();
                    ppm = format!("{}\n{} ", ppm, value);
                    char_count = 0;
                } else {
                    ppm = format!("{}{} ", ppm, value);
                    char_count = next_char_count;
                }
            }
            ppm.pop();
            ppm.push('\n');
        }
        ppm
    }
}

fn scale_color(color: &Color, max: i32) -> [i32; 3] {
    let total_values = max + 1; // include 0 (0..=max is max+1 values)
    [color.red, color.green, color.blue].map(|value| {
        let mut scaled = (value * total_values as f64) as i32;
        if scaled < 0 {
            scaled = 0;
        } else if scaled > max {
            scaled = max;
        }
        scaled
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Color;

    #[test]
    fn scale_color_clamps_values_bewteen_zero_and_max() {
        let result = scale_color(&Color::new(1.5, 0.5, -0.1), 255);
        assert_eq!(result, [255, 128, 0]);

        let result = scale_color(&Color::new(0.9, 0.1, 0.4), 255);
        assert_eq!(result, [230, 25, 102]);
    }

    #[test]
    fn canvas_to_ppm_lines_do_not_exceed_70_chars() {
        let mut canvas = Canvas::new(10, 2);
        for y in 0..2 {
            for x in 0..10 {
                canvas.write_pixel(x, y, Color::new(1.0, 0.8, 0.6));
            }
        }
        let ppm = canvas.to_ppm();
        let expected_ppm = "P3\n\
            10 2\n\
            255\n\
            255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
            153 255 204 153 255 204 153 255 204 153 255 204 153\n\
            255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
            153 255 204 153 255 204 153 255 204 153 255 204 153\n\
            ";
        assert_eq!(ppm, expected_ppm);
    }

    #[test]
    fn canvas_to_ppm_with_pixels() {
        let mut canvas = Canvas::new(5, 3);
        canvas.write_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        canvas.write_pixel(2, 1, Color::new(0.0, 0.5, 0.0));
        canvas.write_pixel(4, 2, Color::new(-0.5, 0.0, 1.0));
        let ppm = canvas.to_ppm();
        let expected_ppm = "P3\n\
            5 3\n\
            255\n\
            255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
            0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
            0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n\
            ";
        assert_eq!(ppm, expected_ppm);
    }

    #[test]
    fn canvas_to_ppm_with_no_pixels_has_header() {
        let canvas = Canvas::new(5, 3);
        let ppm = canvas.to_ppm();
        let header = &ppm.lines().collect::<Vec<&str>>()[0..3];
        assert_eq!(header, ["P3", "5 3", "255"]);
    }

    #[test]
    fn writing_a_pixel_out_of_bounds_of_canvas() {
        let mut canvas = Canvas::new(5, 10);
        canvas.write_pixel(5, 9, Color::new(1.0, 0.0, 0.0));
        assert_eq!(canvas.pixel_at(1, 10), None);
    }

    #[test]
    fn writing_and_getting_a_pixel_on_the_canvas() {
        let mut canvas = Canvas::new(5, 10);
        canvas.write_pixel(2, 3, Color::new(1.0, 0.0, 0.0));
        assert_eq!(canvas.pixel_at(2, 3).unwrap(), &Color::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn creating_a_new_canvas() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        assert_eq!(canvas.pixels.len(), 200);

        for pixel in canvas.pixels {
            assert_eq!(pixel, Color::new(0.0, 0.0, 0.0));
        }
    }
}
