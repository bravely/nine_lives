// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

#![no_std]

use alloc::{format, string::ToString};

extern crate alloc;
extern crate crankstart_sys;
use {
    alloc::boxed::Box,
    anyhow::Error,
    crankstart::{
        crankstart_game,
        geometry::{ScreenPoint, ScreenVector},
        graphics::{Graphics, LCDColor, LCDSolidColor},
        system::System,
        Game, Playdate,
    },
    crankstart_sys::{LCD_COLUMNS, LCD_ROWS},
    euclid::{point2, vec2},
};

struct MessageState {
    location: ScreenPoint,
    velocity: ScreenVector,
}

impl MessageState {
    pub fn new(_playdate: &Playdate) -> Result<Self, Error> {
        Ok(Self {
            location: point2(INITIAL_X, INITIAL_Y),
            velocity: vec2(1, 2),
        })
    }
}

impl Game for MessageState {
    fn update(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        let graphics = Graphics::get();
        graphics.clear(LCDColor::Solid(LCDSolidColor::kColorWhite))?;
        graphics.draw_text("Hello Allison!!!", self.location)?;

        self.location += self.velocity;

        if self.location.x < 0 || self.location.x > LCD_COLUMNS as i32 - TEXT_WIDTH {
            self.velocity.x = -self.velocity.x;
        }

        if self.location.y < 0 || self.location.y > LCD_ROWS as i32 - TEXT_HEIGHT {
            self.velocity.y = -self.velocity.y;
        }

        System::get().draw_fps(0, 0)?;

        Ok(())
    }
}

const INITIAL_X: i32 = (400 - TEXT_WIDTH) / 2;
const INITIAL_Y: i32 = (240 - TEXT_HEIGHT) / 2;

const TEXT_WIDTH: i32 = 86;
const TEXT_HEIGHT: i32 = 16;

struct NineLives {
    message_state: MessageState,
    crank_zero_angle: f32,
    player_in_control: bool,
}

impl NineLives {
    pub fn new(playdate: &Playdate) -> Result<Box<Self>, Error> {
        crankstart::display::Display::get().set_refresh_rate(50.0)?;
        let system = System::get();

        Ok(Box::new(NineLives {
            message_state: MessageState::new(playdate)?,
            crank_zero_angle: system.get_crank_angle()?,
            player_in_control: false,
        }))
    }

    fn check_buttons(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        let system = System::get();
        let (_, pushed, _) = system.get_button_state()?;
        System::log_to_console(&format!("Pushed: {:?}", pushed));
        Ok(())
    }
}

impl Game for NineLives {
    fn update(&mut self, playdate: &mut Playdate) -> Result<(), Error> {
        self.message_state.update(playdate)?;
        self.check_buttons(playdate)?;

        let system = System::get();
        if !system.is_crank_docked()? {
            let diff = system.get_crank_angle()? - self.crank_zero_angle;
            let graphics = Graphics::get();
            graphics.draw_text(&diff.to_string(), point2(INITIAL_X, INITIAL_Y))?;
        }
        {
            self.crank_zero_angle = 0.0;
        }

        Ok(())
    }
}

crankstart_game!(NineLives);
