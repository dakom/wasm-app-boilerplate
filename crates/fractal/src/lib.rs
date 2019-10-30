use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static mut ORIGIN_REAL:f64 = 0.398_890_670_160_257_9;
static mut ORIGIN_IMAGINARY:f64 = 0.312_518_821_896_837_33;
static mut RADIUS:f64 = 0.1;
//static mut ZOOM_FACTOR:f64 = 0.0;
//static mut ZOOM_CENTER_X:f64 = 0.0;
//static mut ZOOM_CENTER_Y:f64 = 0.0;

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn update_pixels(pixels:&mut [u8], palette:&[u8], width:u32, height:u32, max_iterations:u32) {
    unsafe {
        let scale_x:f64= (2.0 * RADIUS) / (width as f64);
            let scale_y:f64= (2.0 * RADIUS) / (height as f64);

            for y in 0..height {
                for x in 0..width {
                    let cr = (x as f64).mul_add(scale_x, ORIGIN_REAL - RADIUS);
                    let ci = (y as f64).mul_add(scale_y, ORIGIN_IMAGINARY - RADIUS);
                    let mut zr = 0.0f64;
                    let mut zi = 0.0f64;

                    for iter in 0..max_iterations { 
                        let zr2 = zr * zr;
                        let zi2 = zi * zi;

                        if (zr2 + zi2) >= 4.0 {
                            put_pixel(pixels, palette, width, height, x, y, iter as usize); 
                            break;
                        }

                        let znr = zr2 - zi2 + cr;
                        let zni = 2.0f64.mul_add(zr * zi, ci);
                        zr = znr;
                        zi = zni;
                    }
                }
            }
    }
}

#[wasm_bindgen]
pub fn cycle_palette(palette:&mut [u8]) {
    let first_color:[u8;4] = [palette[0], palette[1], palette[2], palette[3]];

    let len = palette.len();
    palette.copy_within(4.., 0);
    palette[len-4..].copy_from_slice(&first_color);
}

fn put_pixel(pixels:&mut [u8], palettes:&[u8], width:u32, _height: u32, x:u32, y: u32, color_index: usize) {
    let pixel_offset = (((y * width) + x) * 4) as usize;
    let color_offset = color_index * 4; 

    pixels[pixel_offset] = palettes[color_offset];
    pixels[pixel_offset + 1] = palettes[color_offset + 1];
    pixels[pixel_offset + 2] = palettes[color_offset + 2];
    pixels[pixel_offset + 3] = palettes[color_offset + 3];
}