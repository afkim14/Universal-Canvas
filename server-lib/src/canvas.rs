use std::iter::Iterator;

extern crate rgb;
use self::rgb::*;
extern crate json;
use self::json::*;

use universe::{Universe, Atom};
use json_convertible::*;

/// Canvas struct implements the server side's implementation of the canvas.
/// It keeps track of the width, height, pixels, and the pixel_size to be used to draw the canvas on the client-side.
#[derive(Debug)]
pub struct Canvas {
    /// Width of the canvas as the number of pixels.
    width: usize,
    /// Height of the canvas as the number of pixels.
    height: usize,
    /// Size of a pixel when drawn on the client side.
    pixel_size: usize,
    /// Vector of pixels.
    pixels : Vec<Pixel>
}

impl Canvas {
    pub fn new(width: usize, height: usize, pixel_size: usize) -> Self {
        let pixels = (0..width * height)
            .map(|id| Pixel::new(id))
            .collect();
        Canvas {
            width,
            height,
            pixel_size,
            pixels,
        }
    }
}

impl AsJson for Canvas {
    fn as_json(&self) -> JsonValue {
        let pixels_json: Array = self.pixels
            .iter()
            .map(|p| p.as_json())
            .collect();
        object!{
            "width" => self.width,
            "height" => self.height,
            "pixelSize" => self.pixel_size,
            "pixels" => pixels_json,
        }
    }
}

impl Universe<Pixel> for Canvas {
    fn atom_name_singular(&self) -> &str {
        "pixel"
    }

    fn update_atom(&mut self, pixel: Pixel) {
        // Given a new pixel update, update the canvas
        let id = pixel.id;
        if id >= self.pixels.len() {
            // Error handling or log
            eprint!("Pixel id out of bound: {} >= {}", id, self.pixels.len());
            return;
        }
        self.pixels[id] = pixel;
    }
}

/// The Pixel struct represents a single pixel (square) on the canvas.
#[derive(Debug, PartialEq)]
pub struct Pixel {
    /// The id of the pixel. Useful for updating specific pixels in both the server and client side.
    pub id : usize,

    /// A `RGB8` object of a simple RGB container.
    pub color : RGB8,
}

impl Pixel {
    pub fn new(id: usize) -> Self {
        Pixel {
            id,
            color: RGB8::default(),
        }
    }
}

impl AsJson for Pixel {
    fn as_json(&self) -> JsonValue {
        let color = self.color;
        object!{
            "id" => self.id,
            "color" => object!{
                "r" => color.r,
                "g" => color.g,
                "b" => color.b,
            }
        }
    }
}

impl FromJson for Pixel {
    fn from_json(json: &JsonValue) -> Option<Self> {
        let id = json["id"].as_usize();
        if id.is_none(){
            return None;
        }
        let color = &json["color"];
        let r = color["r"].as_u8();
        let g = color["g"].as_u8();
        let b = color["b"].as_u8();
        if r.is_none() || g.is_none() || b.is_none() {
            return None;
        }
        Some(Pixel {
            id: id.unwrap(),
            color: RGB8::new(r.unwrap(), g.unwrap(), b.unwrap()),
        })
    }
}
