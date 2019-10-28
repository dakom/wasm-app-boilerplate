use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static mut ORIGIN_REAL:f64 = 0.3988906701602579;
static mut ORIGIN_IMAGINARY:f64 = 0.31251882189683733;
static mut RADIUS:f64 = 1.0;
static mut ZOOM_FACTOR:f64 = 0.5;
static mut ZOOM_CENTER_X:f64 = 0.0;
static mut ZOOM_CENTER_Y:f64 = 0.0;

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn update_pixels(pixels:&mut [u8], palettes:&[u8], width:u32, height:u32, max_iterations:u32) {
    unsafe {
        let scaleX:f64= (2.0 * RADIUS) / (width as f64);
            let scaleY:f64= (2.0 * RADIUS) / (height as f64);

            for y in 0..height {
                for x in 0..width {
                    let cr = (ORIGIN_REAL - RADIUS) + ((x as f64) * scaleX);
                    let ci = (ORIGIN_IMAGINARY - RADIUS) + ((y as f64) * scaleY);
                    let mut zr = 0.0;
                    let mut zi = 0.0;

                    for iter in 0..max_iterations { 
                        let zr2 = zr * zr;
                        let zi2 = zi * zi;

                        if (zr2 + zi2) >= 4.0 {
                            put_pixel(pixels, palettes, width, height, x, y, iter as usize); 
                            break;
                        }

                        let znr = zr2 - zi2 + cr;
                        let zni = 2.0 * (zr * zi) + ci;
                        zr = znr;
                        zi = zni;
                    }
                }
            }
    }
}

fn put_pixel(pixels:&mut [u8], palettes:&[u8], width:u32, height: u32, x:u32, y: u32, color_index: usize) {
    let pixel_offset = (((y * width) + x) * 4) as usize;
    let color_offset = (color_index * 4); 

    pixels[pixel_offset] = palettes[color_offset];
    pixels[pixel_offset + 1] = palettes[color_offset + 1];
    pixels[pixel_offset + 2] = palettes[color_offset + 2];
    pixels[pixel_offset + 3] = palettes[color_offset + 3];
}