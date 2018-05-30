extern crate rgb;
use self::rgb::*;
extern crate json;
use self::json::*;
extern crate rand;
use self::rand::*;

#[derive(Debug)]
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
            color: RGB8::new(0, 0, 0), // default color as black
        }
    }
    pub fn change_color(&mut self, newcolor: RGB8) {
        self.color = newcolor;
    }

    pub fn from_json(json: JsonValue) -> Self {
        unimplemented!();
    }

    pub fn stringify(&self) -> String {
        let json_text = object!{
            "id" => self.id,
            "r"  => self.color.r,
            "g"  => self.color.g,
            "b"  => self.color.b
        };
        json_text.dump()
    }
}

#[derive(Debug)]
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
            pixels.push(Pixel::new(id));
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

        let json_text = object! {
            "Title" => REPLY_ENTIRE_BOARD,
            "Width" => self.width,
            "Height" => self.height,
            "PixelSize" => self.pixel_size,
            "Pixels" => pixels_json
        };

        json_text.dump()
    }
}


#[cfg(test)]
mod test_pixel {
    use super::*;

    #[test]
    fn test_change_color() {
        let mut pixel = Pixel::new(5);
        pixel.change_color(RGB8::new(5, 6, 7));
        assert_eq!(pixel.id, 5);
        assert_eq!((pixel.color.r, pixel.color.g, pixel.color.b), (5, 6, 7));
    }

    #[test]
    fn test_stringify_0() {
        let pixel = Pixel::new(1);
        let expected = r#"{"id":1,"r":0,"g":0,"b":0}"#;
        assert_eq!(pixel.stringify(), expected);
    }

    #[test]
    fn test_stringify_1() {
        let mut pixel = Pixel::new(5);
        pixel.change_color(RGB8::new(5, 6, 7));
        let expected = r#"{"id":5,"r":5,"g":6,"b":7}"#;
        assert_eq!(pixel.stringify(), expected);
    }

    #[test]
    fn test_from_json() {
        unimplemented!();
    }

}
