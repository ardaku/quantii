// Copyright (c) 2022 The Quantii Contributors
//
// This file is part of Quantii.
//
// Quantii is free software: you can redistribute
// it and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// Quantii is distributed in the hope that it
// will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU Lesser General Public
// License for more details.
//
// You should have received a copy of the GNU
// Lesser General Public License along with
// Quantii. If not, see <https://www.gnu.org/licenses/>.




#[derive(Clone, Copy)]
pub struct Pixel {
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
}

#[derive(Clone)]
pub struct FrameBuffer {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) bpp: usize,
    pub(crate) buffer: *mut &'static mut [&'static mut [Pixel]],
}

impl FrameBuffer {
    /// Create the framebuffer
    #[must_use]
    pub fn new(width: usize, height: usize, bpp: usize, buffer: &'static mut &'static mut [&'static mut [Pixel]]) -> Self {
        Self {
            width,
            height,
            bpp,
            buffer,
        }
    }

    /// Initialize the framebuffer
    pub fn init(&mut self) {
        let mut x = 0;
        for (y, row) in unsafe {self.buffer.take_mut(..)}.iter_mut().enumerate() {
            for pixel in row.iter() {
                self.draw_pixel(x, y, pixel);
                x += 1;
            }
            x = 0;
        }
    }

    /// Draw a pixel
    pub(crate) fn draw_pixel(&mut self, x: usize, y: usize, pixel: &Pixel) {
        unsafe { (*self.buffer)[y][x] = *pixel; }
    }

    /// Clear the framebuffer
    pub(crate) fn clear(&self) {
        let buffer = unsafe { core::slice::from_raw_parts_mut(0x000B8000 as *mut u8, 4000) };
        for byte in buffer {
            *byte = 0;
        }
    }

    /// Get the framebuffer width
    #[must_use] pub fn get_width(&self) -> usize {
        self.width
    }

    /// Get the framebuffer height
    #[must_use] pub fn get_height(&self) -> usize {
        self.height
    }

    /// Get the framebuffer bpp
    #[must_use] pub fn get_bpp(&self) -> usize {
        self.bpp
    }

    /// Get the framebuffer buffer
    #[must_use] pub fn get_buffer(&mut self) -> *mut &'static mut [&'static mut [Pixel]] {
        self.buffer
    }

    /// Blip the framebuffer
    pub fn blip(&mut self) {
        self.clear();
        let buffer = unsafe {*self.buffer};
        for (y, row) in buffer.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                unsafe { self._draw(x, y, pixel); }
            }
        }
    }

    /// Internal blip function
    unsafe fn _draw(&self, x: usize, y: usize, pixel: &Pixel) {
        // Prepare the screen for drawing
        let screen = core::slice::from_raw_parts_mut(0x000B8000 as *mut u8, 4000);
        // Calculate the offset
        let offset = (y * self.width + x) * 2;
        // Draw the pixel
        screen[offset] = pixel.blue;
        screen[offset + 1] = 0x0F;

    }
}
