use crate::errors::FbError;
use memmap::{MmapMut, MmapOptions};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::ErrorKind;
use std::os::unix::io::AsRawFd;
use std::path::Path;

const FBIOGET_VSCREENINFO: libc::c_ulong = 0x4600;
const FBIOGET_FSCREENINFO: libc::c_ulong = 0x4602;

#[repr(C)]
#[derive(Debug, Default)]
pub struct FbBitfield {
    /// Beginning of bitfield
    offset: u32,
    /// Lenght of bitfield
    length: u32,
    /// Most significant bit is right
    msb_right: u32,
}

impl FbBitfield {
    pub fn apply_bitfield(&self, v: u8) -> u32 {
        let p = (v as u32) >> (8 - self.length) as u32;
        p << self.offset as u32
    }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct FbVarScreenInfo {
    /// Visible resolution x
    pub x_res: u32,
    /// Visible resolution y
    pub y_res: u32,
    /// Virtual resolution x
    pub x_res_virtual: u32,
    /// Virtual resolution y
    pub y_res_virtual: u32,
    /// Offset from virtual to visible resolution x
    pub x_offset: u32,
    /// Offset from virtual to visible resolution y
    pub y_offset: u32,

    /// Bits per pixels
    pub bits_per_pixel: u32,
    /// 0 = Color, 1 = Grayscale, > 1 = fourcc
    pub grayscale: u32,
    /// Red bitfield in fb mem if true color else only length is significant
    pub red: FbBitfield,
    /// Green bitfield in fb mem if true color else only length is significant
    pub green: FbBitfield,
    /// Blue bitfield in fb mem if true color else only length is significant
    pub blue: FbBitfield,
    /// Transparent bitfield in fb mem if true color else only length is significant
    pub transparent: FbBitfield,

    /// != Non standard pixel format
    pub non_std: u32,

    /// See FbActivate*
    pub activate: u32,

    /// Height of picture in mm
    pub height: u32,
    /// Width of picture in mm
    pub width: u32,

    /// (OBSOLETE) see FbInfo.flags
    pub acceleration_flags: u32,

    /// Pixel clock in pico seconds
    pub pixel_clock: u32,
    /// Time (in pixel_clock) from sync to picture
    pub left_margin: u32,
    /// Time (in pixel_clock) from picture to sync
    pub right_margin: u32,
    /// Time (in pixel_clock) from sync to picture
    pub upper_margin: u32,
    /// Time (in pixel_clock) from picture to sync
    pub lower_margin: u32,
    /// Length of horizontal sync
    pub hsync_len: u32,
    /// Length of vertical sync
    pub vsync_len: u32,

    /// See FbSync*
    pub sync: u32,
    /// See FbVmode*
    pub v_mode: u32,
    /// Angle we rotate counter clockwise
    pub rotate: u32,
    /// Colorspace for fourcc-based modes
    pub colorspace: u32,
    /// Reserved for future compatibility
    pub reserved: [u64; 2],
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct FbFixScreenInfo {
    /// Identification string
    pub id: [u8; 16],
    /// Start of framebuffer mem
    pub s_mem_start: libc::c_ulong,
    /// Lenght of framebuffer mem
    pub s_mem_len: u32,
    /// Framebuffer type, see FbType*
    pub fb_type: u32,
    /// Interleave for interleaved planes
    pub type_aux: u32,
    /// See FbVisual*
    pub visual: u32,
    /// Zero if no hardware panning
    pub x_pan_step: u16,
    /// Zero if no hardware panning
    pub y_pan_step: u16,
    /// Zero if no hardware panning
    pub y_wrap_step: u16,
    /// Length of a line in bytes
    pub line_length: u32,
    /// Start of memory mapped i/o
    pub memory_mapped_io_start: libc::c_ulong,
    /// Lenght of memory mapped i/o
    pub memory_mapped_io_len: u32,
    /// Indicate to driver which specific chip/card we have
    pub acceleration: u32,
    /// See FbCap*
    pub capabilities: u16,
    /// Reserved for future compatibility
    pub reserved: [u16; 2],
}

pub struct Framebuffer<'a> {
    path: &'a Path,

    device: File,
    pub frame: MmapMut,

    pub fix_screen_info: FbFixScreenInfo,
    pub var_screen_info: FbVarScreenInfo,
}

impl<'a> Framebuffer<'a> {
    pub fn get_var_screen_info(device: &File) -> Result<FbVarScreenInfo, FbError> {
        let mut var_screen_info: FbVarScreenInfo = Default::default();
        let result = unsafe {
            libc::ioctl(
                device.as_raw_fd(),
                FBIOGET_VSCREENINFO as _,
                &mut var_screen_info,
            )
        };
        match result {
            -1 => Err(FbError::IoctlFailed { code: result }),
            _ => Ok(var_screen_info),
        }
    }

    pub fn get_fix_screen_info(device: &File) -> Result<FbFixScreenInfo, FbError> {
        let mut fix_screen_info: FbFixScreenInfo = Default::default();
        let result = unsafe {
            libc::ioctl(
                device.as_raw_fd(),
                FBIOGET_FSCREENINFO as _,
                &mut fix_screen_info,
            )
        };
        match result {
            -1 => Err(FbError::IoctlFailed { code: result }),
            _ => Ok(fix_screen_info),
        }
    }

    pub fn new(path: &'a Path) -> Result<Self, Box<dyn Error>> {
        let device = OpenOptions::new().write(true).read(true).open(path)?;

        let fix_screen_info = Self::get_fix_screen_info(&device)?;
        let var_screen_info = Self::get_var_screen_info(&device)?;

        let frame_length = (fix_screen_info.line_length * var_screen_info.y_res) as usize;
        let frame = unsafe { MmapOptions::new().len(frame_length).map_mut(&device) }?;
        Ok(Self {
            device,
            frame,
            path,
            fix_screen_info,
            var_screen_info,
        })
    }

    pub fn write_frame(&mut self, frame: &[u8]) -> Result<(), Box<dyn Error>> {
        self.frame[..].copy_from_slice(&frame);
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        match self.frame.flush() {
            Err(e) => {
                if e.raw_os_error() != Some(0) {
                    Err(Box::new(e))
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
}
