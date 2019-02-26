use crate::consts::*;
use crate::display::Displayable;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Clone, Copy)]
pub struct Ball {
    position: Rect,
    vel_x: i32,
    vel_y: i32,
    color: Color,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            position: Rect::new((WIDTH / 2) as i32, (HEIGHT / 2) as i32, SQUARE_SIZE, SQUARE_SIZE), 
            vel_x: 5,
            vel_y: 5,
            color: Color::RGB(255, 255, 255),
        }
    }

    pub fn advance(&mut self, left_player: &Player, right_player: &Player) {
        let new_x = self.position.x() + self.vel_x;
        let new_y = self.position.y() + self.vel_y;

        self.position.set_x(new_x);
        self.position.set_y(new_y);

        if self.position.has_intersection(left_player.position) || self.position.has_intersection(right_player.position) {
            self.vel_x *= -1;
        }

        if new_y < 0 || new_y + SQUARE_SIZE as i32 > HEIGHT as i32 {
            self.vel_y *= -1;
        }
    }

    pub fn is_oob(&self) -> bool {
        self.position.x() < 0 || self.position.x() + SQUARE_SIZE as i32 > WIDTH as i32
    }

    pub fn speed_up(&mut self) {
        if self.vel_x > 0 {
            self.vel_x += 1;
        } else {
            self.vel_x -= 1;
        }
        if self.vel_y > 0 {
            self.vel_y += 1;
        } else {
            self.vel_y -= 1;
        }
    }
}

impl Displayable for Ball {
    fn into_boxes(&self) -> (Color, Rect) {
        (self.color, self.position)
    }
}


#[derive(Clone, Copy)]
pub struct Player {
    position: Rect,
    color: Color,
}

impl Player {
    pub fn new_left() -> Player {
        Player {
            position: Rect::new(SQUARE_SIZE as i32, ((HEIGHT - 15 * SQUARE_SIZE) / 2) as i32, SQUARE_SIZE, 15 * SQUARE_SIZE),
            color: Color::RGB(255, 255, 255),
        }
    }

    pub fn new_right() -> Player {
        Player {
            position: Rect::new((WIDTH - 2 * SQUARE_SIZE) as i32, ((HEIGHT - 15 * SQUARE_SIZE) / 2) as i32, SQUARE_SIZE, 15 * SQUARE_SIZE),
            color: Color::RGB(255, 255, 255),
        }
    }

    pub fn move_up(&mut self) {
        let new_position = self.position.y() - MOVING_SPEED;
        if new_position < 0 {
            self.position.set_y(0);
        } else {
            self.position.set_y(new_position);
        }
    }

    pub fn move_down(&mut self) {
        let new_position = self.position.y() + MOVING_SPEED;
        if new_position > (HEIGHT - 15 * SQUARE_SIZE) as i32 {
            self.position.set_y((HEIGHT - 15 * SQUARE_SIZE) as i32);
        } else {
            self.position.set_y(new_position);
        }
    }
}

impl Displayable for Player {
    fn into_boxes(&self) -> (Color, Rect) {
        (self.color, self.position)
    }
}
