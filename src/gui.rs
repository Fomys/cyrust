use crate::buffer::Buffer;
use crate::framebuffer::{FbVarScreenInfo, Framebuffer};
use std::error::Error;
use std::path::Path;

#[derive(Debug, Default, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const GREEN: Color = Color {
        r: 0,
        g: 255,
        b: 0,
        a: 0,
    };
    pub const RED: Color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 0,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 255,
        a: 0,
    };
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    fn to_bitfield(self, fb_var_screen_info: &FbVarScreenInfo) -> Vec<u8> {
        let c = fb_var_screen_info.red.apply_bitfield(self.r)
            | fb_var_screen_info.green.apply_bitfield(self.g)
            | fb_var_screen_info.blue.apply_bitfield(self.b)
            | fb_var_screen_info.transparent.apply_bitfield(self.a);
        c.to_le_bytes()[0..fb_var_screen_info.bits_per_pixel as usize / 8].to_vec()
    }
}

pub struct FbGUI<'a> {
    framebuffer: Framebuffer<'a>,
}

impl<'a> FbGUI<'a> {
    pub fn new(path: &'a Path) -> Result<Self, Box<dyn Error>> {
        let framebuffer = Framebuffer::new(path)?;
        Ok(Self { framebuffer })
    }

    pub fn horizontal_line(&mut self, y: usize, color: Color) {
        let c = color.to_bitfield(&self.framebuffer.var_screen_info);
        let line_length = self.framebuffer.fix_screen_info.line_length as usize;
        self.framebuffer.frame[y * line_length..((y + 1) * line_length)].copy_from_slice(
            (0..line_length)
                .map(|i| c[i % c.len()])
                .collect::<Vec<u8>>()
                .as_ref(),
        );
    }

    pub fn part_horizontal_line(&mut self, y: usize, x_start: usize, x_stop: usize, color: Color) {
        let c = color.to_bitfield(&self.framebuffer.var_screen_info);
        println!("{:?}", c);
        let line_length = self.framebuffer.fix_screen_info.line_length as usize;
        let opp = (self.framebuffer.var_screen_info.bits_per_pixel / 8) as usize;
        self.framebuffer.frame[y * line_length + x_start * opp..(y * line_length + (x_stop) * opp)]
            .copy_from_slice(
                (0..(x_stop - x_start) * opp)
                    .map(|i| c[i % c.len()])
                    .collect::<Vec<u8>>()
                    .as_ref(),
            );
    }

    pub fn vertical_line(&mut self, x: usize, color: Color) {
        let c = color.to_bitfield(&self.framebuffer.var_screen_info);
        let opp = (self.framebuffer.var_screen_info.bits_per_pixel / 8) as usize;
        let line_length = self.framebuffer.fix_screen_info.line_length as usize;
        let x = x * opp;
        for i in 0..self.framebuffer.var_screen_info.y_res as usize {
            let p = i * line_length + x;
            self.framebuffer.frame[p..p + opp].copy_from_slice(c.as_slice());
        }
    }

    pub fn part_vertical_line(&mut self, x: usize, y_start: usize, y_stop: usize, color: Color) {
        let c = color.to_bitfield(&self.framebuffer.var_screen_info);
        let opp = (self.framebuffer.var_screen_info.bits_per_pixel / 8) as usize;
        let line_length = self.framebuffer.fix_screen_info.line_length as usize;
        let x = x * opp;
        for i in y_start..y_stop {
            let p = i * line_length + x;
            self.framebuffer.frame[p..p + opp].copy_from_slice(c.as_slice());
        }
    }

    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        self.framebuffer.update()
    }

    pub fn fill(&mut self, color: Color) {
        let c = color.to_bitfield(&self.framebuffer.var_screen_info);
        let fb_length = self.framebuffer.fix_screen_info.line_length as usize
            * self.framebuffer.var_screen_info.y_res as usize;
        self.framebuffer.frame[0..fb_length].copy_from_slice(
            (0..fb_length)
                .map(|i| c[i % c.len()])
                .collect::<Vec<u8>>()
                .as_ref(),
        );
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let opp = (self.framebuffer.var_screen_info.bits_per_pixel / 8) as usize;
        let line_length = self.framebuffer.fix_screen_info.line_length as usize;
        self.framebuffer.frame[line_length * y + x..line_length * y + x + opp].copy_from_slice(
            color
                .to_bitfield(&self.framebuffer.var_screen_info)
                .as_slice(),
        )
    }

    pub fn write_buf(&mut self, x_offset: usize, y_offset: usize, buf: &Buffer) {
        for x in 0..buf.size.x {
            for y in 0..buf.size.y {
                self.set_pixel(x + x_offset, y + y_offset, buf.content[y * buf.size.x + x])
            }
        }
    }
}
