use crankstart::geometry::ScreenCoord;

use {
    anyhow::Error,
    crankstart::{
        geometry::{ScreenPoint, ScreenVector},
        graphics::{Graphics, LCDColor, LCDSolidColor},
        system::System,
        Game, Playdate,
    },
    crankstart_sys::{LCD_COLUMNS, LCD_ROWS},
    euclid::{point2, vec2},
};

pub struct MessageState {
    location: ScreenPoint,
    velocity: ScreenVector,
    text_width: i32,
    pub player_in_control: bool,
}

pub const TEXT_WIDTH: i32 = 86;
pub const TEXT_HEIGHT: i32 = 16;
pub const INITIAL_X: i32 = (400 - TEXT_WIDTH) / 2;
pub const INITIAL_Y: i32 = (240 - TEXT_HEIGHT) / 2;

const FONTPATH: &str = "/System/Fonts/Asheville-Sans-14-Bold.pft";
const MESSAGE: &str = "Hello Alli";

impl MessageState {
    pub fn new(_playdate: &Playdate) -> Result<Self, Error> {
        let graphics = Graphics::get();
        Ok(Self {
            location: point2(INITIAL_X, INITIAL_Y),
            velocity: vec2(1, 2),
            text_width: graphics.get_text_width(&graphics.load_font(FONTPATH)?, MESSAGE, 0)?,
            player_in_control: false,
        })
    }

    pub fn stop_for_player(&mut self) {
        if self.player_in_control {
            return;
        }
        self.player_in_control = true;
        self.velocity = vec2(0, 0);
    }

    pub fn reset(&mut self) {
        if !self.player_in_control {
            return;
        }
        self.player_in_control = false;
        self.velocity = vec2(1, 2);
    }

    pub fn change_velocity(&mut self, x: ScreenCoord, y: ScreenCoord) {
        if !self.player_in_control {
            return;
        }
        self.velocity += vec2(x, y);
    }
}

impl Game for MessageState {
    fn update(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        let graphics = Graphics::get();
        graphics.clear(LCDColor::Solid(LCDSolidColor::kColorWhite))?;
        System::get().draw_fps(0, 0)?;

        self.location += self.velocity;

        if self.location.x < 0 || self.location.x > LCD_COLUMNS as i32 - self.text_width {
            self.velocity.x = -self.velocity.x;
        }

        if self.location.y < 0 || self.location.y > LCD_ROWS as i32 - TEXT_HEIGHT {
            self.velocity.y = -self.velocity.y;
        }

        graphics.draw_text(MESSAGE, self.location)?;

        Ok(())
    }
}
