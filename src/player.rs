use doryen_rs::{Color, DoryenApi, ScanCode};

use crate::config::{PLAYER_SPEED, WHITE};

use super::entity::Entity as Player;
/*
pub struct Player {
    pos: (f32, f32),
    speed: f32,
}
*/

impl Player {
    /*
    pub fn new(speed: f32) -> Self {
        Self {
            pos: (0.0, 0.0),
            speed,
        }
    }
    */
    pub fn new_player(pos: (i32, i32)) -> Self {
        Self {
            ch: '@' as u16,
            pos,
            name: "Player".to_owned(),
            color: (WHITE),
            light: true,
            speed: PLAYER_SPEED,
        }
    }
    // player movement from keyboard
    pub fn move_from_input(& self, api: &mut dyn DoryenApi) -> (i32, i32) {
        let input = api.input();
        let mut mov = (0, 0);

        mov.0 = match (input.key(ScanCode::Left) || input.key(ScanCode::A), 
                       input.key(ScanCode::Right) || input.key(ScanCode::D)) {
            (true, _,) => -1,
            (_, true) => 1,
            _ => 0,
        };
    
        mov.1 = match (input.key(ScanCode::Up) || input.key(ScanCode::W), 
                       input.key(ScanCode::Down) || input.key(ScanCode::S)) {
            (true, _,) => -1,
            (_, true) => 1,
            _ => 0,
        };
        mov
    }
    // move player to cords
    pub fn move_to(&mut self, pos: (i32, i32)) {
        self.pos = (pos.0 /*as f32 */, pos.1 /*as f32 */);
    }
    // move by some value
    pub fn move_by(&mut self, mov: (i32, i32), /*coef: f32, */ api: &mut dyn DoryenApi) -> bool {
        let oldx =self.pos.0 as i32;
        let oldy =self.pos.1 as i32;

        if api.input().key(ScanCode::LShift) {
            // sprint by LShift
            self.pos.0 += self.speed * 2 * mov.0 /*as f32 * coef */;
            self.pos.1 += self.speed * 2 * mov.1 /*as f32 * coef */;
        } else {
            self.pos.0 += self.speed * mov.0 /*as f32 * coef */;
            self.pos.1 += self.speed * mov.1 /*as f32 * coef */;
        }
        oldx == self.pos.0 as i32 && oldy == self.pos.1 as i32
    }
    // return the next position (x + mov.x, y + mov.y) of player
    pub fn next_pos(&self, mov: (i32, i32)) -> (i32, i32) {
        (self.pos.0 as i32 + mov.0, self.pos.1 as i32 + mov.1)
    }
    // return position (x,y) of player
    pub fn pos(&self) -> (i32, i32) {
        (self.pos.0 as i32, self.pos.1 as i32)
    }
    // render player
    /*
    pub fn render(&self, api: &mut dyn DoryenApi, light: Color) {
        let con = api.con();
        let pos = self.pos();
        con.ascii(pos.0, pos.1, '@' as u16);
        con.fore(pos.0, pos.1, light);
    }
     */
}