use std::borrow::Borrow;

use crate::game::Game;

use super::player::Player;

pub struct Ball {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    xspeed: i8,
    yspeed: i8,
    pub score: u8,
}

impl Ball {
    pub fn new(x: f64, y: f64, radius: f64, xspeed: i8, yspeed: i8, score: u8) -> Self {
        Self {
            x,
            y,
            radius,
            xspeed,
            yspeed,
            score,
        }
    }

    pub fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("white"));
        ctx.begin_path();
        ctx.arc(self.x, self.y, self.radius, 0.0, 2.0 * std::f64::consts::PI);
        ctx.fill();
    }

    pub fn update(
        &mut self,
        player1: std::cell::Ref<Player>,
        player2: std::cell::Ref<Player>,
        canvas_width: u32,
        canvas_height: u32,
    ) -> usize {
        self.x += self.xspeed as f64;
        self.y += self.yspeed as f64;

        self.check_collision_with_player(player1.borrow().clone(), player2.borrow().clone());
        self.check_collision_with_wall(canvas_width, canvas_height)
    }

    pub fn check_collision_with_player(&mut self, player1: &Player, player2: &Player) {
        if self.x - self.radius < player1.x + player1.width
            && self.y - self.radius < player1.y + player1.height
            && self.y + self.radius > player1.y && self.x + self.radius > player1.x
        {
            self.xspeed = -self.xspeed;
        }

        if self.x + self.radius > player2.x
            && self.y - self.radius < player2.y + player2.height
            && self.y + self.radius > player2.y && self.x - self.radius < player2.x + player2.width
        {
            self.xspeed = -self.xspeed;
        }
    }
    pub fn check_collision_with_wall(&mut self, canvas_width: u32, canvas_height: u32) -> usize {
        if self.y + self.radius > canvas_height as f64 || self.y - self.radius < 0.0 {
            self.yspeed = -self.yspeed;
        }

        if self.x + self.radius > canvas_width as f64 {
            self.reset_position(canvas_width, canvas_height);
            return 1;
        }

        if self.x - self.radius < 0.0 {
            self.reset_position(canvas_width, canvas_height);
            return 2;
        }

        return 0;
    }

    fn reset_position(&mut self, canvas_width: u32, canvas_height: u32) {
        self.x = canvas_width as f64 / 2.0;
        self.y = canvas_height as f64 / 2.0;
    }
}
