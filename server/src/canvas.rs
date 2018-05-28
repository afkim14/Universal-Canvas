extern crate rgb;
use self::rgb::*;
extern crate json;
use self::json::*;

pub struct Pixel {
    pub id : usize, // id or position
    pub color : RGB8,
    // pub author : String, // if we want to keep track of who modified the pixel
}

impl Pixel {
    pub fn new(id: usize) -> Self {
        // Default Constructor
        Pixel {
            id,
            color: RGB8::new(0, 0, 0),
        }
    }
    pub fn change_color(&mut self, newcolor: RGB8) {
        self.color = newcolor;
    }

    pub fn from_json(json: JsonValue) -> Self {
        unimplemented!();
    }
}

pub struct Canvas {
    pub pixels : Vec<Pixel> // 2D Vec? Hashmap<pixid, pixel>?
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        // Default Constructor
        // Canvas { pixels: vec![] }
        let mut pixels = vec![];
        for id in 0..width * height {
            pixels.push(Pixel::new(id));
        }
        Canvas { pixels }
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

    pub fn serialize(&self) -> String {
        unimplemented!();
    }
}
