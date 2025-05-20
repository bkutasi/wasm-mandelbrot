use wasm_bindgen::prelude::*;
use log::{info, Level}; // Keep log for potential debugging
pub use wasm_bindgen_rayon::init_thread_pool; // Re-export for JS
use rayon::prelude::*; // Import Rayon prelude

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).unwrap();
    Ok(())
}

#[wasm_bindgen]
pub fn draw(width: u32, height: u32, zoom: f64, offsetX: f64, offsetY: f64, resolution: f64) -> Vec<u8> {
    let mut data = vec![0u8; (width * height * 4) as usize];

    let f_width = width as f64;
    let f_height = height as f64;
    let step = (1.0 / resolution).max(1.0) as usize; // usize for step_by

    // Number of rows to process in each band.
    // Ensure it's a multiple of `step` for easier processing, or handle remainders carefully.
    // For simplicity, let's make band_row_step align with `step`.
    // This means each y_base in a band will be a multiple of `step`.
    let band_rows_unaligned = (height as usize).max(1) / rayon::current_num_threads().max(1);
    let band_rows = ((band_rows_unaligned + step -1) / step) * step; // Ensure multiple of step, or at least step
    let band_rows = band_rows.max(step);


    data.par_chunks_mut((width as usize * band_rows * 4).max(1)) // Each chunk is a band of rows
        .enumerate()
        .for_each(|(band_index, band_slice)| {
            let y_band_start_row = band_index * band_rows; // Starting row index for this band

            for y_base_offset in (0..band_rows).step_by(step) {
                let y_base = y_band_start_row as u32 + y_base_offset as u32;
                if y_base >= height { continue; } // Ensure y_base is within canvas height

                for x_base in (0..width as i32).step_by(step) { // Iterating over canvas width with step
                    let fx = x_base as f64 + (step as f64 - 1.0) / 2.0;
                    let fy = y_base as f64 + (step as f64 - 1.0) / 2.0;

                    let mut zx = 1.5 * (fx - f_width / 2.0) / (0.5 * zoom * f_width) + offsetX;
                    let mut zy = (fy - f_height / 2.0) / (0.5 * zoom * f_height) + offsetY;

                    let cX = zx;
                    let cY = zy;
                    let mut iter = 0;

                    while (zx * zx + zy * zy < 4.0) && iter < 1000 {
                        let tmp = zx * zx - zy * zy + cX;
                        zy = 2.0 * zx * zy + cY;
                        zx = tmp;
                        iter += 1;
                        if zx == tmp && zy == cY { iter = 1000; break; }
                    }

                    let color_val = if iter < 999 { (iter % 256) as u8 } else { 0 };

                    for y_offset_in_block in 0..step {
                        let py_canvas = y_base + y_offset_in_block as u32;
                        if py_canvas >= height { continue; }

                        for x_offset_in_block in 0..step {
                            let px_canvas = x_base as u32 + x_offset_in_block as u32;
                            if px_canvas >= width { continue; }

                            // Calculate index within the current band_slice
                            // y_local is y_base_offset + y_offset_in_block
                            // py_canvas is the absolute y on the canvas
                            // y_band_slice_relative is py_canvas - y_band_start_row
                            let y_band_slice_relative = py_canvas - y_band_start_row as u32;
                            
                            // Check if this pixel actually belongs to the current band_slice
                            // This check is important if band_rows doesn't perfectly divide height
                            // or if y_base + y_offset_in_block goes beyond the band's designated rows
                            // but still within canvas height.
                            if y_band_slice_relative < band_rows as u32 {
                                let band_slice_index = ((y_band_slice_relative * width + px_canvas) * 4) as usize;
                                
                                // Boundary check for band_slice itself
                                if band_slice_index + 3 < band_slice.len() {
                                    band_slice[band_slice_index] = color_val;
                                    band_slice[band_slice_index + 1] = color_val;
                                    band_slice[band_slice_index + 2] = color_val;
                                    band_slice[band_slice_index + 3] = 255;
                                }
                            }
                        }
                    }
                }
            }
        });
    data
}

#[wasm_bindgen]
pub fn free_buffer(ptr: *mut u8, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, capacity);
    }
}

