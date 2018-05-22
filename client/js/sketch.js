var canvas;
var constrainedCanvasWidth = 1250;
var constrainedCanvasHeight = 650;

function setup() {
	canvas = createCanvas(constrainedCanvasWidth, constrainedCanvasHeight);
	canvas.parent('canvas');
	background(Math.floor(Math.random() * 256), Math.floor(Math.random() * 256), Math.floor(Math.random() * 256));
    //draw_board();
}

function draw() {

}

function draw_board() { 
    var pixelSize = 20;
    for (var i = 0; i < constrainedCanvasWidth / pixelSize; i++) {
        for (var j = 0; j < constrainedCanvasHeight / pixelSize; j++) {            
	        var newButton = createButton('');
	        newButton.id("pixel");
            newButton.position(i*pixelSize, j*pixelSize);
        }
    }
}

function keyPressed() {
	if (keyCode == 32) {
		location.reload();
	}
}
