extern crate image;

use image::{GenericImageView,RgbaImage};
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let img = image::open(&args[1]).unwrap();
    //let dimensions = img.dimensions();

    for pixel in img.pixels() {
            for y in (pixel.1 - 1)..(pixel.1 + 2) {
                ret.concat(&mut img.get_pixel(x, y))
            }
        }
    }
}

