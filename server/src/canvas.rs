extern crate rgb;
use self::rgb::*;
extern crate json;
use self::json::*;
extern crate rand;
use self::rand::*;

pub struct Pixel {
    pub id : usize, // id or position
    pub color : RGB8,
    // pub author : String, // if we want to keep track of who modified the pixel
}

impl Pixel {
    pub fn new(id: usize, r: u8, g: u8, b: u8) -> Self {
        // Default Constructor
        Pixel {
            id,
            color: RGB8::new(r, g, b) // for some reason can't put r g b here
        }
    }
    pub fn change_color(&mut self, newcolor: RGB8) {
        self.color = newcolor;
    }

    pub fn from_json(json: JsonValue) -> Self {
        unimplemented!();
    }

    pub fn stringify(&self) -> String {
        let data = object! {
            "id" => self.id,
            "r" => self.color.r as usize,
            "g" => self.color.g as usize,
            "b" => self.color.b as usize
        };

        data.dump()
    }
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixel_size: usize,
    pub pixels : Vec<Pixel> // 2D Vec? Hashmap<pixid, pixel>?
}

// REPLY CONSTANTS
const REPLY_ENTIRE_BOARD :&str = "REPLY_ENTIRE_BOARD";

impl Canvas {
    pub fn new(width: usize, height: usize, pixel_size: usize) -> Self {
        // Default Constructor
        let mut pixels = vec![];
        for id in 0..width * height {
            // let r = thread_rng().gen_range(0, 200);
            // let g = thread_rng().gen_range(0, 200);
            // let b = thread_rng().gen_range(0, 200);
            pixels.push(Pixel::new(id, 0, 0, 0));
        }
        Canvas { width, height, pixel_size, pixels }
    }
    pub fn new_from_file() -> Self {
        // Build canvas from saved file
        unimplemented!();
    }
    pub fn update_pixel(pixel: Pixel) {
        // Given a new pixel update, update the canvas
        unimplemented!();
    }

    #[allow(dead_code)]
    pub fn show_board() {
        // Print canvas just for testing purposes
        unimplemented!();
    }

    pub fn stringify(&self) -> String {
        // maybe not want to do it this way
        let mut pixels_json = json::JsonValue::new_array();
        for p in &self.pixels {
            pixels_json.push(p.stringify());
        }

        let data = object! {
            "Title" => REPLY_ENTIRE_BOARD,
            "Width" => self.width,
            "Height" => self.height,
            "PixelSize" => self.pixel_size,
            "Pixels" => pixels_json
        };

        data.dump()
    }
}
