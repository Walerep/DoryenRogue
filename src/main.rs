extern crate doryen_rs;
extern crate doryen_fov;

mod player;
mod entity;
mod level;
mod light;
mod noise;

use std::borrow::BorrowMut;

use doryen_rs::{App, AppOptions, Color, Console, DoryenApi, Engine, TextAlign, UpdateEvent};

use entity::Entity;
use level::Level;
use player::Player;

// consts
const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;
const GAME_WIDTH: u32 = CONSOLE_WIDTH - 10;
const GAME_HEIGHT: u32 = CONSOLE_HEIGHT;
const HUD_WIDTH: u32 = CONSOLE_WIDTH - GAME_WIDTH - 1;
const HUD_HEIGHT: u32 = CONSOLE_HEIGHT - 2;
const PLAYER_SPEED: f32 = 0.2;
const PLAYER_FOV_RADIUS: usize = 40;

// colors
const WHITE: Color = (255, 255, 255, 255);
const BLACK: Color = (0, 0, 0, 255);
const RED: Color = (255, 92, 92, 255);
const BLUE: Color = (192, 192, 255, 255);

/* main struct of game
   must include player, entities, level etc. */
struct DoryenRogue {
    player: Player,
    entities: Vec<Entity>,
    mouse_pos: (f32, f32),
    level: Level,
    loaded: bool,
    game: Console,
    hud: Console,
    alpha: f32,
}

impl DoryenRogue {
    pub fn new() -> Self {
        let mut game = Console::new(GAME_WIDTH, GAME_HEIGHT);
        let mut hud = Console::new(HUD_WIDTH, HUD_HEIGHT);
        for y in 0..HUD_HEIGHT as i32 {
            for x in 0..HUD_WIDTH as i32 {
                hud.back(x, y, RED);
            }
        }
        hud.print((HUD_WIDTH / 2) as i32, 0, "SOME HUD", TextAlign::Center, None, None);

        game.print((GAME_WIDTH - 1) as i32, (GAME_WIDTH / 2) as i32, "Game", TextAlign::Center, None, None);

        Self {
            player: Player::new(PLAYER_SPEED),
            entities: Vec::new(),
            mouse_pos: (0.0, 0.0),
            level: Level::new("src/level"),
            loaded: false,
            game: game,
            hud: hud,
            alpha: 1.0,
        }
    }
    fn render_entities(&self, _api: &mut dyn DoryenApi) {
        for entity in self.entities.iter() {
            if self.level.is_in_fov(entity.pos) {
                entity.render(_api, &self.level);
            }
        }
        let player_pos = self.player.pos();
        let player_light = self.level.light_at(player_pos);
        self.player.render(_api, player_light);
    }
    fn render_entities_to_blitz(&self, _api: &mut dyn DoryenApi, con: &mut Console) {
        for entity in self.entities.iter() {
            if self.level.is_in_fov(entity.pos) {
                entity.render_to_blitz(&self.game, &self.level);
            }
        }
        let player_pos = self.player.pos();
        let player_light = self.level.light_at(player_pos);
        self.player.render(_api, player_light);
    }
    fn clear_con(&self, _api: &mut dyn DoryenApi) {
        let con = _api.con();
        con.clear(Some(BLACK), Some(BLACK), Some(' ' as u16));
    }
}

impl Engine for DoryenRogue {
    // game first one-time init
    fn init(&mut self, _api: &mut dyn DoryenApi) {
        _api.con().register_color("white", WHITE);
        _api.con().register_color("black", BLACK);
        _api.con().register_color("red", RED);
        _api.con().register_color("blue", BLUE);
    }

    // world and logic update
    fn update(&mut self, _api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        if !self.loaded {
            if let Some(entities) = self.level.try_load() {
                self.loaded = true;
                self.player.move_to(self.level.start_pos());
                self.level.compute_fov(self.player.pos(), PLAYER_FOV_RADIUS);
                self.entities = entities;
            }
        }
        if self.loaded {
            let mut coef = 1.0 / std::f32::consts::SQRT_2;
            let mut mov = self.player.move_from_input(_api);
            if self.level.is_wall(self.player.next_pos((mov.0, 0))) {
                mov.0 = 0;
                coef = 1.0;
            }
            if self.level.is_wall(self.player.next_pos((0, mov.1))) {
                mov.1 = 0;
                coef = 1.0;
            }
            if self.player.move_by(mov, coef, _api) {
                self.level.compute_fov(self.player.pos(), PLAYER_FOV_RADIUS);
            }
            self.mouse_pos = _api.input().mouse_pos();
            self.level.update();
        }
        None
    }

    fn render(&mut self, _api: &mut dyn DoryenApi) {
        if self.loaded {
            self.clear_con(_api);
            //self.level.render(_api, self.player.pos());
            self.level.render_to_blitz(&mut self.game, self.player.pos());
            //self.render_entities(_api);
            self.render_entities_to_blitz(_api, self.game);
            let fps = _api.fps();
            _api.con().print_color(
                (CONSOLE_WIDTH / 2) as i32, 
                (CONSOLE_HEIGHT - 2) as i32, 
                &format!(
                    "#[white]Move with #[red]arrows or WASD #[white] {:4} fps", fps
                ), 
                TextAlign::Center,
                None,
            );
        } else {
            _api.con().print_color(
                (CONSOLE_WIDTH / 2) as i32, 
                (CONSOLE_HEIGHT / 2) as i32, 
                "#[white] Loading#[red]....", 
                TextAlign::Center, 
                None,
            );
        }
        self.game.blit(
            0, 
            0, 
            _api.con(), 
            self.alpha, 
            self.alpha, 
            None);
        self.hud.blit(
            (CONSOLE_WIDTH - HUD_WIDTH - 1) as i32, 
            (CONSOLE_HEIGHT - HUD_HEIGHT - 1) as i32, 
            _api.con(), 
            self.alpha, 
            self.alpha, 
            None);
    }
}

fn main() {
    let mut app = App::new(AppOptions {
        window_title: "doryen rogue".to_owned(),
        vsync: false,
        ..Default::default()
    });
    app.set_engine(Box::new(DoryenRogue::new()));
    app.run();
}
