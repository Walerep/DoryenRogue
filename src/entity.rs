use doryen_rs::{color_mul, color_scale, Color, DoryenApi, Console};

use crate::level::Level;
use crate::light::{Light, LIGHT_COEF};

// basic struct of entity
pub struct Entity {
    // ascii char for this entity
    ch: u16,
    // position on map (x, y)
    pub pos: (i32, i32),
    pub name: String,
    color: Color,
    light: bool,
}

impl Entity {
    // create new goblin entity
    pub fn new_goblin(pos: (i32, i32)) -> Self {
        Self {
            ch: 'g' as u16,
            pos,
            name: "goblin".to_owned(),
            color: (80, 15, 70, 255),
            light: false,
        }
    }
    // create new light entity
    pub fn new_light(pos: (i32, i32)) -> Self {
        Self {
            ch: 15,
            pos,
            name: "a torch".to_owned(),
            color: (150, 174, 27, 255),
            light: true,
        }
    }
    // render entity and give it a light if it should
    pub fn render(&self, api: &mut dyn DoryenApi, level: &Level) {
        let (color, penumbra) = if self.light {
            (self.color, false)
        } else {
            let light = level.light_at(self.pos);
            let penumbra = Light::is_penumbra(light, 100);
            let mut color = color_mul(self.color, light);
            if penumbra {
                color = color_scale(color, LIGHT_COEF);
            }
            (color, penumbra)
        };
        api.con().ascii(self.pos.0, 
            self.pos.1, 
            if penumbra { '?' as u16 } else { self.ch },
        );
        api.con().fore(self.pos.0, self.pos.1, color);
    }
    // render (TO BLITZ CON) entity and give it a light if it should
    pub fn render_to_blitz(&self, con: &Console, level: &Level) {
        let (color, penumbra) = if self.light {
            (self.color, false)
        } else {
            let light = level.light_at(self.pos);
            let penumbra = Light::is_penumbra(light, 100);
            let mut color = color_mul(self.color, light);
            if penumbra {
                color = color_scale(color, LIGHT_COEF);
            }
            (color, penumbra)
        };
        let mut cons = con;
        cons.ascii(self.pos.0, 
            self.pos.1, 
            if penumbra { '?' as u16 } else { self.ch },
        );
        cons.fore(self.pos.0, self.pos.1, color);
    }
}