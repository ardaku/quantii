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

use core::slice::from_raw_parts_mut;

#[cfg(feature = "rpi")]
pub mod mailbox;

#[cfg(feature = "rpi")]
pub use mailbox::*;

#[derive(Clone, Copy)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Clone)]
pub struct FrameBuffer<const WIDTH: usize, const HEIGHT: usize> {
    pub(crate) bpp: usize,
    pub(crate) buffer: *mut [[Pixel; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> FrameBuffer<WIDTH, HEIGHT> {
    /// Create the framebuffer
    #[must_use]
    pub fn new(
        bpp: usize,
        buffer: *mut [[Pixel; WIDTH]; HEIGHT],
    ) -> Self {
        Self{
            bpp,
            buffer,
        }
    }

    /// Initialize the framebuffer
    pub fn init(&mut self) {
        let mut x = 0;
        for (y, row) in
            unsafe { *self.buffer }.iter_mut().enumerate()
        {
            for pixel in row.iter() {
                self.draw_pixel(x, y, pixel);
                x += 1;
            }
            x = 0;
        }
    }

    /// Draw a pixel
    pub(crate) fn draw_pixel(&mut self, x: usize, y: usize, pixel: &Pixel) {
        unsafe {
            (*self.buffer)[y][x] = *pixel;
        }
    }

    /// Clear the framebuffer
    pub(crate) fn clear(&self) {
        let buffer = unsafe {
            from_raw_parts_mut(0x000B8000 as *mut u8, 4000)
        };
        for byte in buffer {
            *byte = 0;
        }
    }

    /// Get the framebuffer WIDTH
    #[must_use]
    pub fn get_width(&self) -> usize {
        WIDTH
    }

    /// Get the framebuffer HEIGHT
    #[must_use]
    pub fn get_height(&self) -> usize {
        HEIGHT
    }

    /// Get the framebuffer bpp
    #[must_use]
    pub fn get_bpp(&self) -> usize {
        self.bpp
    }

    /// Get the framebuffer buffer
    #[must_use]
    pub fn get_buffer(&mut self) -> *mut [[Pixel; WIDTH]; HEIGHT] {
        self.buffer
    }

    /// Blip the framebuffer
    pub fn blip(&mut self) {
        self.clear();
        let buffer = unsafe { *self.buffer };
        for (y, row) in buffer.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                unsafe {
                    // self._draw(x, y, pixel);
                }
            }
        }
    }

    // /// Internal draw function
    // unsafe fn _draw(&self, x: usize, y: usize, pixel: &Pixel) {
    //     // Prepare the screen for drawing
    //     let screen = FB_MEMORY_LOCATION as *mut u8;
    //     // Calculate the offset
    //     let offset = (y * WIDTH + x) * 2;
    //     // Draw the pixel
    //     screen.write_volatile( pixel.red);
    //     screen.add(1).write_volatile(pixel.green);
    //     screen.add(2).write_volatile(pixel.blue);
    //     screen.add(3).write_volatile(pixel.red);
    // }
}
