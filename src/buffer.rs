use crate::Color;
use cgmath::Vector2;
use image::{GenericImageView, Pixel};

const W: Color = Color::WHITE;
const R: Color = Color::RED;
const B: Color = Color::BLACK;
pub struct Buffer {
    pub(crate) content: Vec<Color>,
    pub(crate) size: Vector2<usize>,
}

impl Buffer {
    pub fn new(size: Vector2<usize>, color: Color) -> Self {
        Self {
            size,
            content: vec![color; size.x * size.y],
        }
    }

    pub fn heart() -> Buffer {
        Buffer {
            #[rustfmt::skip]
            content: vec![
                W, W, B, B, W, W, W, B, B, W, W,
                W, B, R, R, B, W, B, R, R, B, W,
                B, R, R, R, R, B, R, R, R, R, B,
                B, R, R, R, R, R, R, R, R, R, B,
                W, B, R, R, R, R, R, R, R, B, W,
                W, W, B, R, R, R, R, R, B, W, W,
                W, W, W, B, R, R, R, B, W, W, W,
                W, W, W, W, B, R, B, W, W, W, W,
                W, W, W, W, W, B, W, W, W, W, W,
            ],
            size: (11, 9).into(),
        }
    }

    pub fn scale(&self, scale: usize) -> Buffer {
        let mut new_content = vec![Color::default(); self.size.x * scale * self.size.y * scale];

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                for x_bis in 0..scale {
                    for y_bis in 0..scale {
                        new_content[(y * scale + y_bis) * self.size.x * scale + x * scale + x_bis] =
                            self.content[y * self.size.x + x]
                    }
                }
            }
        }

        Buffer {
            content: new_content,
            size: (self.size.x * scale, self.size.y * scale).into(),
        }
    }

    pub fn put_pixel(&mut self, p: Vector2<usize>, color: Color) {
        self.content[p.y as usize * self.size.x + p.x as usize] = color;
    }

    pub fn draw_line(&mut self, mut start: Vector2<usize>, stop: Vector2<usize>, color: Color) {
        let dx = (stop.x as isize - start.x as isize).abs();
        let sx = start.x < stop.x;
        let dy = -(stop.y as isize - start.y as isize).abs();
        let sy = start.y < stop.y;
        let mut err = dx + dy;
        loop {
            self.put_pixel(start, color);
            if start == stop {
                break;
            };
            let e2 = 2 * err;
            if e2 > dy {
                err += dy;
                if sx {
                    start.x += 1;
                } else {
                    start.x -= 1;
                }
            }
            if e2 < dx {
                err += dx;
                if sy {
                    start.y += 1;
                } else {
                    start.y -= 1;
                }
            }
        }
    }

    #[cfg(all(target_endian = "big"))]
    pub fn from_u8(c: [u8; 8]) -> Self {
        let mut new_buffer = vec![Color::default(); 64];
        for x in 0..8 {
            for y in 0..8 {
                if (c[y] & (0b0000_0001 << (8 - x)) as u8) != 0 {
                    new_buffer[y * 8 + x] = Color::BLACK;
                } else {
                    new_buffer[y * 8 + x] = Color::WHITE;
                }
            }
        }
        Self {
            content: new_buffer,
            size: (8, 8),
        }
    }

    #[cfg(all(target_endian = "little"))]
    pub fn from_u8(c: [u8; 8]) -> Self {
        let mut new_buffer = vec![Color::default(); 64];
        for x in 0..8 {
            for y in 0..8 {
                if (c[y] & (0b0000_0001 << x) as u8) != 0 {
                    new_buffer[y * 8 + x] = Color::BLACK;
                } else {
                    new_buffer[y * 8 + x] = Color::WHITE;
                }
            }
        }
        Self {
            content: new_buffer,
            size: (8, 8).into(),
        }
    }
}

impl<T, P> From<T> for Buffer
where
    P: Pixel<Subpixel = u8>,
    T: GenericImageView<Pixel = P>,
{
    fn from(image: T) -> Self {
        let size = image.dimensions();
        let mut new_content = vec![Color::default(); (size.0 * size.1) as usize];
        for x in 0..size.0 {
            for y in 0..size.1 {
                let c = image.get_pixel(x, y).to_rgba();

                new_content[y as usize * size.0 as usize + x as usize] = Color {
                    r: c.0[0],
                    g: c.0[1],
                    b: c.0[2],
                    a: c.0[3],
                };
            }
        }
        Self {
            content: new_content,
            size: (size.0 as usize, size.1 as usize).into(),
        }
    }
}
