use wasm_bindgen::prelude::*; // Importing the prelude from wasm_bindgen, which contains all the necessary traits and types.
use wasm_bindgen::JsCast; // Importing JsCast trait for performing conversions between JavaScript types and Rust.
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement}; // Importing specific types from web_sys for working with canvas.
use log::{info, Level};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).unwrap();
    Ok(())
}

#[wasm_bindgen] // This attribute indicates that the following function is accessible from JavaScript.
pub fn draw(canvas: HtmlCanvasElement, zoom: f64, offsetX: f64, offsetY: f64, resolution: f64) { // The draw function takes a canvas element and parameters for zoom and offsets.
    let context = canvas
        .get_context("2d") // Getting a 2D rendering context from the canvas.
        .unwrap() // Unwrapping the Result.
        .unwrap() // Unwrapping the Option.
        .dyn_into::<CanvasRenderingContext2d>() // Dynamically casting into CanvasRenderingContext2d.
        .unwrap(); // Unwrapping the Result.

    let width = canvas.width() as f64; // Getting the width of the canvas and converting to f64.
    let height = canvas.height() as f64; // Getting the height of the canvas and converting to f64.

    context.clear_rect(0.0, 0.0, width, height); // Clearing the entire canvas.

    for x in (0..width as i32).step_by((1.0 / resolution) as usize) { // Iterating over each pixel in the width of the canvas.
        for y in (0..height as i32).step_by((1.0 / resolution) as usize) { // Iterating over each pixel in the height of the canvas.
            let mut zx = 1.5 * (x as f64 - width / 2.0) / (0.5 * zoom * width) + offsetX; // Calculating zx based on x, zoom and offsetX.
            let mut zy = (y as f64 - height / 2.0) / (0.5 * zoom * height) + offsetY; // Calculating zy based on y, zoom and offsetY.

            let cX = zx; // Storing initial zx value in cX.
            let cY = zy; // Storing initial zy value in cY.
            let mut iter = 0; // Initializing iteration count.

            while (zx * zx + zy * zy < 4.0) && iter < 1000 { // While loop for Mandelbrot set calculation until max iterations or escape radius is reached.
                let tmp = zx * zx - zy * zy + cX; 
                zy = 2.0 * zx * zy + cY;
                zx = tmp;
                iter += 1;

                // Early bailout
                if zx == tmp && zy == cY {
                    iter = 1000;
                    break;
                }
            }

            let color = if iter < 999 { iter % 256 } else { 0 }; // Determining color based on iteration count.
            context.set_fill_style(&JsValue::from_str(&format!("rgb({},{},{})", color, color, color))); // Setting fill style with determined color.
            context.fill_rect(x as f64, y as f64, resolution, resolution); // Filling a rectangle at (x,y) with size 1x1.
        }
    }
}

