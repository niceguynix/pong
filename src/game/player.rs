pub struct Player {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub speed: i8,
    pub score: u8,
}

impl Player {
    pub fn new(x: f64, y: f64, width: f64, height: f64, speed: i8, score: u8) -> Self {
        Player {
            x,
            y,
            width,
            height,
            speed,
            score,
        }
    }

    pub fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("white"));
        ctx.fill_rect(self.x, self.y, self.width, self.height);
    }
}
