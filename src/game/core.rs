use super::ball::Ball;
use super::player::Player;
use crate::utils::{animate_limited, console_log, log, KeyBoardListener};
use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;

pub struct Game {
    player1: Rc<RefCell<Player>>,
    player2: Rc<RefCell<Player>>,
    ball: Ball,
    score1: u32,
    score2: u32,
    canvas_height: u32,
    canvas_width: u32,
    renderingCtx: web_sys::CanvasRenderingContext2d,
}

impl Game {
    pub fn init(canvas: web_sys::HtmlCanvasElement) -> Self {
        let renderingCtx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        let canvas_height = canvas.height();
        let canvas_width = canvas.width();
        let player1 = Player::new(
            canvas_width as f64 * 0.05,
            canvas_height as f64 / 2.0,
            10.0,
            100.0,
            10,
            0,
        );
        let player2 = Player::new(
            canvas_width as f64 * 0.95,
            canvas_height as f64 / 2.0,
            10.0,
            100.0,
            10,
            0,
        );
        let ball = Ball::new(10.0, 10.0, 10.0, 10, 10, 0);
        let score1 = 0;
        let score2 = 0;
        Game {
            player1: Rc::new(RefCell::new(player1)),
            player2: Rc::new(RefCell::new(player2)),
            ball,
            score1,
            score2,
            canvas_height,
            canvas_width,
            renderingCtx,
        }
    }

    fn set_callbacks(&self) {
        let p1 = Rc::clone(&self.player1);
        let p2 = Rc::clone(&self.player2);
        KeyBoardListener(Closure::new(move |key: web_sys::KeyboardEvent| {
            
            let key = key.key();
            let mut p2 = p2.borrow_mut();
            let mut p1 = p1.borrow_mut();
            
            match key.as_str() {
                "ArrowUp" => p2.y-=10.0,
                "ArrowDown" => p2.y+=10.0,
                "w" => p1.y-=10.0,
                "s" => p1.y+=10.0,
                _ => (),
            }
        }));
    }

    pub fn start(self) {
        self.set_callbacks();
        self.game_loop();
    }

    fn game_loop(self) {
        animate_limited(
            |game: &mut Game| {
                game.update();
                game.render();
            },
            self,
            60,
        )
    }

    fn update(&mut self) {
        match self.ball.update(
            self.player1.borrow(),
            self.player2.borrow(),
            self.canvas_width,
            self.canvas_height,
        ) {
            1 => self.score1 += 1,
            2 => self.score2 += 1,
            _ => (),
        }
    }

    fn render(&self) {
        console_log!("Hi");
        self.renderingCtx.clear_rect(
            0.0,
            0.0,
            self.canvas_width as f64,
            self.canvas_height as f64,
        );
        self.fill_canvas();
        self.player1.borrow().draw(&self.renderingCtx);
        self.player2.borrow().draw(&self.renderingCtx);
        self.ball.draw(&self.renderingCtx);
        self.draw_score();
    }

    fn fill_canvas(&self) {
        self.renderingCtx
            .set_fill_style(&wasm_bindgen::JsValue::from_str("black"));
        self.renderingCtx.fill_rect(
            0.0,
            0.0,
            self.canvas_width as f64,
            self.canvas_height as f64,
        );
    }

    fn draw_score(&self) {
        let x = 0.05;
        let y = 0.1;
        let score1 = "Score: ".to_owned() + &self.score1.to_string().to_owned();
        let score2 = "Score: ".to_owned() + &self.score2.to_string().to_owned();

        self.renderingCtx
            .set_fill_style(&wasm_bindgen::JsValue::from_str("white"));
        self.renderingCtx.set_font("15px Arial");
        self.renderingCtx
            .fill_text(&score1, (self.canvas_width as f64*x)-15.0, self.canvas_height as f64 *y)
            .unwrap();
        self.renderingCtx
            .fill_text(&score2, (self.canvas_width as f64*(1.0-x))-15.0, self.canvas_height as f64 *y)
            .unwrap();
    }
}
