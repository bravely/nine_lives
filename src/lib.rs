#![no_std]

mod character;
mod message_state;

use alloc::string::ToString;
use character::Character;
use crankstart_sys::PDButtons;

extern crate alloc;
extern crate crankstart_sys;
use {
    alloc::boxed::Box,
    anyhow::Error,
    crankstart::{
        crankstart_game, graphics::Graphics, sprite::Sprite, system::System, Game, Playdate,
    },
    euclid::point2,
    message_state::{MessageState, INITIAL_X, INITIAL_Y},
};

struct NineLives {
    message_state: MessageState,
    crank_zero_angle: f32,
    character: Character,
}

macro_rules! any_pressed {
    ( $pushed:ident, $button:expr$(, $rest:expr ),* ) => ( ($pushed & $button) == $button $(|| any_pressed!($pushed, $rest))* );
}

impl NineLives {
    pub fn new(playdate: &Playdate) -> Result<Box<Self>, Error> {
        crankstart::display::Display::get().set_refresh_rate(50.0)?;
        let system = System::get();

        Ok(Box::new(NineLives {
            message_state: MessageState::new(playdate)?,
            crank_zero_angle: system.get_crank_angle()?,
            character: Character::new()?,
        }))
    }

    fn check_buttons(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        let system = System::get();
        let (_, pushed, _) = system.get_button_state()?;
        if any_pressed!(pushed, PDButtons::kButtonA) {
            self.message_state.stop_for_player();
        }
        if any_pressed!(pushed, PDButtons::kButtonB) {
            self.message_state.reset();
        }
        if any_pressed!(pushed, PDButtons::kButtonUp) {
            self.message_state.change_velocity(0, -1);
        } else if any_pressed!(pushed, PDButtons::kButtonDown) {
            self.message_state.change_velocity(0, 1);
        }
        if any_pressed!(pushed, PDButtons::kButtonLeft) {
            self.message_state.change_velocity(-1, 0);
        } else if any_pressed!(pushed, PDButtons::kButtonRight) {
            self.message_state.change_velocity(1, 0);
        }
        Ok(())
    }
}

impl Game for NineLives {
    fn update(&mut self, playdate: &mut Playdate) -> Result<(), Error> {
        self.message_state.update(playdate)?;
        self.check_buttons(playdate)?;
        self.character.ensure()?;

        let system = System::get();
        if !system.is_crank_docked()? {
            let diff: i32 = (system.get_crank_angle()? - self.crank_zero_angle) as i32;
            let graphics = Graphics::get();
            graphics.draw_text(&diff.to_string(), point2(INITIAL_X, INITIAL_Y))?;
        }
        {
            self.crank_zero_angle = 0.0;
        }

        Ok(())
    }

    fn update_sprite(&mut self, sprite: &mut Sprite, playdate: &mut Playdate) -> Result<(), Error> {
        Ok(())
    }
}

crankstart_game!(NineLives);
