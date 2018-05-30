use std::result;

extern crate rgb;
use self::rgb::*;
extern crate json;
use self::json::*;
extern crate rand;
use self::rand::*;


#[derive(Debug, PartialEq)]
pub struct Pixel {
    pub id : usize, // id or position
    pub color : RGB8,
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

    pub fn from_json(json: &JsonValue) -> Option<Self> {
        // TODO: inconsistent pixel representation on client
        let id = json["id"].as_usize();
        let color = &json["color"];
        if id.is_none() || !color.is_object() {
            return None;
        }
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

    pub fn jsonfy(&self) -> JsonValue {
        object!{
            "id" => self.id,
            "r"  => self.color.r,
            "g"  => self.color.g,
            "b"  => self.color.b
        }
    }

    pub fn stringify(&self) -> String {
        self.jsonfy().dump()
    }
}

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixel_size: usize,  // size of a pixel when get drawn on client side
    pub pixels : Vec<Pixel> // emulated 2D vector
}

// REPLY CONSTANTS
const REPLY_ENTIRE_BOARD :&str = "REPLY_ENTIRE_BOARD";

impl Canvas {
    pub fn new(width: usize, height: usize, pixel_size: usize) -> Self {
        // Default Constructor
        let length = width * height;
        let mut pixels = Vec::with_capacity(length);
        for id in 0..length {
            pixels.push(Pixel::new(id));
        }
        Canvas { width, height, pixel_size, pixels }
    }

    pub fn update_pixel(&mut self, pixel: Pixel) {
        // Given a new pixel update, update the canvas
        let id = pixel.id;
        if id >= self.pixels.len(){
            // Error handling or log
            eprint!("Pixel id out of bound: {} >= {}", id, self.pixels.len());
            return;
        }
        self.pixels[id] = pixel;
    }


    // pub fn stringify(&self) -> String {
    //     let mut pixel_array = JsonValue::new_array();
    //     for pixel in self.pixels.iter() {
    //         pixel_array.push(pixel.jsonfy()).expect("Error in creating json file");
    //     }

    //     let json_text = object!{
    //         "width"  => self.width,
    //         "height" => self.height,
    //         "pixels" => pixel_array
    //     };
    //     json_text.dump()
    // }

    pub fn stringify(&self) -> String {
        let mut pixels_json = JsonValue::new_array();
        for p in &self.pixels {
            pixels_json.push(p.jsonfy()).expect("Error in creating json file");
        }

        let json_text = object! {
            "Title"     => REPLY_ENTIRE_BOARD,
            "Width"     => self.width,
            "Height"    => self.height,
            "PixelSize" => self.pixel_size,
            "Pixels"    => pixels_json
        };

        json_text.dump()
    }

    #[allow(dead_code)]
    pub fn new_from_file() -> Self {
        // Build canvas from saved file
        unimplemented!();
    }
}

impl From<Pixel> for json::JsonValue{
    fn from(pixel:Pixel) -> JsonValue {
        pixel.jsonfy()
    }
}


#[cfg(test)]
mod test_canvas {
    use super::*;

    #[test]
    fn test_change_color() {
        let mut pixel = Pixel::new(5);
        pixel.change_color(RGB8::new(5, 6, 7));
        assert_eq!(pixel.id, 5);
        assert_eq!((pixel.color.r, pixel.color.g, pixel.color.b), (5, 6, 7));
    }

    #[test]
    fn test_pixel_stringify_0() {
        let pixel = Pixel::new(1);
        let expected = r#"{"id":1,"r":0,"g":0,"b":0}"#;
        assert_eq!(pixel.stringify(), expected);
    }

    #[test]
    fn test_pixel_stringify_1() {
        let mut pixel = Pixel::new(5);
        pixel.change_color(RGB8::new(5, 6, 7));
        let expected = r#"{"id":5,"r":5,"g":6,"b":7}"#;
        assert_eq!(pixel.stringify(), expected);
    }

    #[test]
    fn test_from_json_ok() {
        let mut pixel = Pixel::new(5);
        pixel.change_color(RGB8::new(5, 6, 7));
        let input = json::parse(&pixel.stringify()).unwrap();
        let parsed_pixel = Pixel::from_json(&input).unwrap();
        assert_eq!(parsed_pixel, pixel);
    }

    // #[test]
    // fn test_from_json_err() {

    // }

    #[test]
    fn test_canvas_stringify() {
        let expected = r#"{"Title":"REPLY_ENTIRE_BOARD","Width":2,"Height":2,"PixelSize":4,"Pixels":[{"id":0,"r":0,"g":0,"b":0},{"id":1,"r":0,"g":0,"b":0},{"id":2,"r":0,"g":0,"b":0},{"id":3,"r":0,"g":0,"b":0}]}"#;
        let canvas = Canvas::new(2, 2, 4);
        assert_eq!(canvas.stringify(), expected);

    }


}
