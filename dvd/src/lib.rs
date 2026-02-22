use rand::random_range;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

const SPEED: f64 = 1.35;

struct App {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    canvas_w: f64,
    canvas_h: f64,
    dvd: HtmlImageElement,
    dvd_w: f64,
    dvd_h: f64,
    x: f64,
    xspeed: f64,
    y: f64,
    yspeed: f64,
    r: i32,
    g: i32,
    b: i32,
}

impl App {
    fn resize(&mut self) {
        let size = size_from_viewport();
        self.canvas_w = size.width;
        self.canvas_h = size.height;

        self.canvas.set_width(self.canvas_w as u32);
        self.canvas.set_height(self.canvas_h as u32);
    }

    fn update(&mut self) {
        self.x += self.xspeed;
        self.y += self.yspeed;

        if self.x + self.dvd_w >= self.canvas_w {
            self.recolor();
            self.x = self.canvas_w - self.dvd_w;
            self.xspeed = -SPEED;
        } else if self.x <= 0.0 {
            self.recolor();
            self.x = 0.0;
            self.xspeed = SPEED;
        }
        if self.y + self.dvd_h >= self.canvas_h {
            self.recolor();
            self.y = self.canvas_h - self.dvd_h;
            self.yspeed = -SPEED;
        } else if self.y <= 0.0 {
            self.recolor();
            self.y = 0.0;
            self.yspeed = SPEED;
        }
    }

    fn draw(&self) {
        self.ctx.clear_rect(0.0, 0.0, self.canvas_w, self.canvas_h);
        self.ctx
            .draw_image_with_html_image_element(&self.dvd, self.x, self.y)
            .unwrap();

        // Set blending mode
        self.ctx
            .set_global_composite_operation("source-atop")
            .unwrap();

        // Set tint color
        self.ctx
            .set_fill_style_str(&format!("rgb({0}, {1}, {2})", self.r, self.g, self.b));

        // Draw tint rectangle over image bounds
        self.ctx.fill_rect(self.x, self.y, self.dvd_w, self.dvd_h);

        // Reset blending mode
        self.ctx
            .set_global_composite_operation("source-over")
            .unwrap();
    }

    fn recolor(&mut self) {
        self.r = random_range(50..255);
        self.g = random_range(50..255);
        self.b = random_range(50..255);
    }
}

struct Size {
    width: f64,
    height: f64,
}

fn size_from_viewport() -> Size {
    let vp = window().unwrap().visual_viewport().unwrap();
    Size {
        width: vp.width(),
        height: vp.height(),
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let win = window().unwrap();
    let document = win.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let canvas_w = win.inner_width().unwrap().as_f64().unwrap();
    let canvas_h = win.inner_height().unwrap().as_f64().unwrap();

    let dvd_logo = document
        .get_element_by_id("dvd-logo")
        .unwrap()
        .dyn_into::<HtmlImageElement>()?;

    let dvd_w = dvd_logo.natural_width() as f64;
    let dvd_h = dvd_logo.natural_height() as f64;

    let app = Rc::new(RefCell::new(App {
        canvas,
        ctx,
        canvas_w,
        canvas_h,
        x: random_range(0.0..(canvas_w - dvd_w)),
        y: random_range(0.0..(canvas_h - dvd_h)),
        xspeed: SPEED,
        yspeed: SPEED,
        dvd: dvd_logo,
        dvd_w,
        dvd_h,
        r: random_range(50..255),
        g: random_range(50..255),
        b: random_range(50..255),
    }));

    // Initial resize
    app.borrow_mut().resize();

    // Resize listener — forgotten intentionally so it lives for the page lifetime
    let app_resize = app.clone();
    let on_resize = Closure::<dyn Fn()>::new(move || app_resize.borrow_mut().resize());
    window()
        .unwrap()
        .add_event_listener_with_callback("resize", on_resize.as_ref().unchecked_ref())?;
    on_resize.forget();

    // Animation loop — the closure holds an Rc to itself so it can reschedule
    let cb: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    *cb.borrow_mut() = Some(Closure::<dyn FnMut()>::new({
        let app = app.clone();
        let cb = cb.clone();
        move || {
            app.borrow_mut().update();
            app.borrow().draw();
            window()
                .unwrap()
                .request_animation_frame(cb.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .unwrap();
        }
    }));

    window()
        .unwrap()
        .request_animation_frame(cb.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;

    Ok(())
}
