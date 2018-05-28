
// Bunch of globals
var canvas;
var constrainedCanvasWidth = 1250;
var constrainedCanvasHeight = 650;
var pixelSize = 10;
var pixels = []; // instead of an array, maybe we want to have some sort of hash? So that we can change individual pixels in O(1)
var currentCursorPos = { x: 0, y: 0 };
var currentPixelIndex = 0;

class Pixel {
    constructor(x, y, size, color) {
        this.x = x;
        this.y = y;
        this.size = size;
        this.color = color;
    }
}


function setup() {
	canvas = createCanvas(constrainedCanvasWidth, constrainedCanvasHeight);
	canvas.parent('canvas');
	background(Math.floor(Math.random() * 256), Math.floor(Math.random() * 256), Math.floor(Math.random() * 256));
    create_canvas();

    var socket = new WebSocket("ws://127.0.0.1:8080");
    setTimeout(
            function () {
                if (socket.readyState === 1) {
                    console.log("Connection is made")
                    socket.send("ayyyy");
                    return;

                } else {
                    console.log("wait for connection...")
                    waitForSocketConnection(socket, callback);
                }

            }, 5);
}

function draw() {
    // p5js makes it so that this is constantly called in a "draw" loop
    draw_canvas();
    show_current_cursor();
}

function create_canvas() {
    // right now i'm just creating it with random values, but we should get the values from the server
    for (var i = 0; i < constrainedCanvasWidth / pixelSize; i++) {
        for (var j = 0; j < constrainedCanvasHeight / pixelSize; j++) {
            var color = {
                r : Math.floor(Math.random() * 256),
                g : Math.floor(Math.random() * 256),
                b : Math.floor(Math.random() * 256)
            };
            pixels.push(new Pixel(i*pixelSize, j*pixelSize, pixelSize, color));
        }
    }
}

function draw_canvas() {
    for (var i = 0; i < pixels.length; i++)  {
        push();
        noStroke();
        fill(pixels[i].color.r, pixels[i].color.g, pixels[i].color.b);
        rect(pixels[i].x, pixels[i].y, pixels[i].size, pixels[i].size);
        pop();
    }
}

function show_current_cursor() {
    push();
    stroke(0);
    strokeWeight(4);
    fill(0, 0, 0, 0);
    rect(currentCursorPos.x, currentCursorPos.y, pixelSize, pixelSize);
    pop();
}

function keyPressed() {
	if (keyCode == 39 && currentCursorPos.x < constrainedCanvasWidth) {
         currentCursorPos.x += pixelSize;
         currentPixelIndex++;
    }
    if (keyCode == 37 && currentCursorPos.x > 0) {
         currentCursorPos.x -= pixelSize;
         currentPixelIndex--;
    }
	if (keyCode == 38 && currentCursorPos.y > 0) {
         currentCursorPos.y -= pixelSize;
         currentPixelIndex -= constrainedCanvasWidth / pixelSize;
    }
    if (keyCode == 40 && currentCursorPos.y < constrainedCanvasHeight) {
        currentCursorPos.y += pixelSize;
        currentPixelIndex += constrainedCanvasWidth / pixelSize;
    }

    // Spacebar
    if (keyCode == 32) {
        // just to show that you can get the current pixel
        console.log(pixels[currentPixelIndex].color);
    }
}
