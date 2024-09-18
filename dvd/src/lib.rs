mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{console, window, CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    init_canvas().unwrap();
}

fn init_canvas() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let canvas = document.create_element("canvas").unwrap();

    canvas.set_attribute("style", "border: 1px solid")?;
    canvas.set_attribute("width", "600")?;
    canvas.set_attribute("height", "300")?;
    // window.inner_width().unwrap(),
    // window.inner_height().unwrap(),

    body.append_child(&canvas)?;

    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let mut dvd = DvDLogo::new();
    // Clear the canvas
    context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
    dvd.draw(context);
    dvd.x += dvd.x_speed;

    // window.request_animation_frame()
    Ok(())
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    alert(&format!("Hello, {s}!"));
}

#[wasm_bindgen]
pub struct Color {
    red: i32,
    green: i32,
    blue: i32,
}

#[wasm_bindgen]
impl Color {
    fn new() -> Color {
        Color {
            red: 100,
            green: 100,
            blue: 100,
        }
    }

    // fn random_color(&mut self) {
    //     self.red = rand::thread_rng().gen_range(50..255);
    //     self.green = rand::thread_rng().gen_range(50..255);
    //     self.blue = rand::thread_rng().gen_range(50..255);
    // }
}

#[wasm_bindgen]
pub struct DvDLogo {
    x: f64,
    y: f64,
    x_speed: f64,
    y_speed: f64,
    width: f64,
    height: f64,
    color: Color,
    image: String,
}

#[wasm_bindgen]
impl DvDLogo {
    fn new() -> DvDLogo {
        DvDLogo {
            x: 10.0,
            y: 5.0,
            x_speed: 3.5,
            y_speed: 3.5,
            width: 120.0,
            height: 80.0,
            color: Color::new(),
            image: String::from("hello"),
        }
    }

    fn draw(&self, context: CanvasRenderingContext2d) {
        context.begin_path();
        context.set_fill_style(&JsValue::from_str("green"));
        context.fill_rect(self.x, self.y, self.width, self.height);
    }
}
