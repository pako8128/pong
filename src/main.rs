mod consts;
mod display;
mod objects;

use self::display::Display;
use self::objects::Ball;
use self::objects::Player;

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut timer = sdl_context.timer().unwrap();
    let mut display = Display::new(&video_subsystem);

    let mut ball = Ball::new();
    let mut left_player = Player::new_left();
    let mut right_player = Player::new_right();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        for key in event_pump.keyboard_state().pressed_scancodes() {
            match key {
                Scancode::W => left_player.move_up(),
                Scancode::S => left_player.move_down(),
                Scancode::Up => right_player.move_up(),
                Scancode::Down => right_player.move_down(),
                _ => {},
            }
        }

        ball.advance(&left_player, &right_player);

        if ball.is_oob() {
            ball = Ball::new();
        }

        // if timer.ticks() % 100 == 0 {
        //     ball.speed_up();
        // }

        display.clear();
        display.render(ball);
        display.render(left_player);
        display.render(right_player);
        display.present();
    }
}
