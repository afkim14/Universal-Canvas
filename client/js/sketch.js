
// Bunch of globals
var canvas;
var pixels = []; // instead of an array, maybe we want to have some sort of hash? So that we can change individual pixels in O(1)
var currPixelIndex = 0;
var currSelectedColor = { r: 255, g: 255, b: 255 };
var PIXEL_SIZE = 0;
var CONSTRAINED_CANVAS_WIDTH = 0;
var CONSTRAINED_CANVAS_HEIGHT = 0;

const RETRIEVE_BOARD = "RETRIEVE_BOARD"
const REPLY_ENTIRE_BOARD = "REPLY_ENTIRE_BOARD";
const PIXEL_CHANGED = "PIXEL_CHANGED"; // we're going to have to fix the spec for requests from client to server

var socket = new WebSocket("ws://127.0.0.1:8080");

// JQUERY (idk where to put this to make it look clean)
$(".color_picker").spectrum({
    color: "#fff",
    change: function(color) {
        currSelectedColor = { r: Math.floor(color._r), g: Math.floor(color._g), b: Math.floor(color._b) };
    }
});

function Pixel(id, x, y, size, color) {
    this.id = id;
    this.x = x;
    this.y = y;
    this.size = size;
    this.color = color;

    this.show = function() {
      push();
      noStroke();
      if (this.id == currPixelIndex) {
        stroke(255);
        strokeWeight(3); // stroke does not look nice rn
      }
      fill(this.color.r, this.color.g, this.color.b);
      rect(this.x, this.y, this.size, this.size);
      pop();
    }
}


function setup() {
    handle_socket_connection();
}

function handle_socket_connection() {
  setTimeout(
          function () {
              if (socket.readyState === 1) {
                  console.log("Connection is made")
                  socket.send(RETRIEVE_BOARD);
                  return;
              } else {
                  console.log("wait for connection...")
                  waitForSocketConnection(socket, callback);
              }
          }, 5);

  // message handlers
  socket.onmessage = function (event) {
    var msg = JSON.parse(event.data);
    if (msg["Title"] == REPLY_ENTIRE_BOARD) {
        PIXEL_SIZE = msg["PixelSize"];
        CONSTRAINED_CANVAS_WIDTH = msg["Width"] * PIXEL_SIZE;
        CONSTRAINED_CANVAS_HEIGHT = msg["Height"] * PIXEL_SIZE;
        create_canvas(msg["Pixels"]);
    }
    // else if (msg["Title"] == .......)
  }
}

function draw() {
    // p5js makes it so that this is constantly called in a "draw" loop
    //show_current_cursor();
}

function create_canvas(json_pixels) {
    canvas = createCanvas(CONSTRAINED_CANVAS_WIDTH, CONSTRAINED_CANVAS_HEIGHT);
    canvas.parent('canvas');
    background(255);

    var currIndex = 0;
    for (var i = 0; i < CONSTRAINED_CANVAS_WIDTH / PIXEL_SIZE; i++) {
        for (var j = 0; j < CONSTRAINED_CANVAS_HEIGHT / PIXEL_SIZE; j++) {
            var parsed_json_pixel = json_pixels[currIndex]; // JSON.parse(json_pixels[currIndex]);
            var color = {
                r : parsed_json_pixel["r"],
                g : parsed_json_pixel["g"],
                b : parsed_json_pixel["b"]
            };
            pixels.push(new Pixel(parsed_json_pixel["id"],
                                  i*PIXEL_SIZE,
                                  j*PIXEL_SIZE,
                                  PIXEL_SIZE,
                                  color
                                ));
            currIndex++;
        }
    }

    draw_canvas();
}

function draw_canvas() {
    for (var i = 0; i < pixels.length; i++)  {
      pixels[i].show();
    }
}

// function show_current_cursor() {
//     push();
//     stroke(255);
//     strokeWeight(4);
//     fill(0, 0, 0, 0);
//     rect(currentCursorPos.x, currentCursorPos.y, PIXEL_SIZE, PIXEL_SIZE);
//     pop();
// }

// rip indentation???
function keyPressed() {
  if (keyCode == 39) { currPixelIndex += CONSTRAINED_CANVAS_HEIGHT / PIXEL_SIZE; } // RIGHT
  if (keyCode == 37) { currPixelIndex -= CONSTRAINED_CANVAS_HEIGHT / PIXEL_SIZE; } // LEFT
	if (keyCode == 38) { currPixelIndex--; }                                         // UP
  if (keyCode == 40) { currPixelIndex++; }                                         // DOWN

  // Spacebar
  if (keyCode == 32) {
      pixels[currPixelIndex].color = currSelectedColor;
      socket.send(JSON.stringify({
          pixel_changed: pixels[currPixelIndex]
      }));
  }

  draw_canvas();
}
