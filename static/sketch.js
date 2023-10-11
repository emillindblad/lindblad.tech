let x,y,xwidth,yheight,xspeed,yspeed,r,g,b;

function preload() {
    dvd = loadImage("https://raw.githubusercontent.com/emillindblad/ghpagestest/master/assets/dvd.png")
}

function pickColor() {
    r = random(50,255)
    g = random(50,255)
    b = random(50,255)
}

function setup() {
    createCanvas(windowWidth, windowHeight)
    xwidth = 120;
    yheight = 80;
    x = random(0, width - xwidth);
    y = random(0, height - yheight);
    xspeed = 3.5;
    yspeed = 3.5;
    pickColor();
}

function draw() {
    background(18,18,18)
    tint(r,g,b)
    image(dvd, x, y, xwidth, yheight)
    x += xspeed;
    y += yspeed;
    if (x + xwidth >= width  || x <= 0 ) {
        xspeed = -xspeed;
        pickColor();
    }
    if (y + yheight >= height || y <= 0 ) {
        yspeed = -yspeed;
        pickColor();
    }
}
