use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::VideoSubsystem;

use crate::consts::*;

pub trait Displayable {
    fn into_boxes(&self) -> (Color, Rect);
}

pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub fn new(video: &VideoSubsystem) -> Self {
        let window = video
            .window("PONG", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();

        Display { canvas }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn render<D: Displayable>(&mut self, object: D) {
        let (color, rect) = object.into_boxes();
            self.canvas.set_draw_color(color);
            self.canvas.fill_rect(rect).unwrap();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
