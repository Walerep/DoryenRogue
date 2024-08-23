
use doryen_rs::{Console, DoryenApi, Engine, TextAlign, UpdateEvent};

use super::config::{
    CONSOLE_WIDTH, CONSOLE_HIEGHT,
    HUD_WIDTH, HUD_HEIGHT, 
    PLAYER_FOV_RADIUS,
    WHITE, BLACK, RED, BLUE,
};

use super::entity::Entity;
use super::entity::Entity as Player;
use super::level::Level;

/* main struct of game
   must include player, entities, level etc. */
pub struct DoryenRogue {
    player: Player,
    entities: Vec<Entity>,
    mouse_pos: (f32, f32),
    level: Level,
    loaded: bool,
    hud: Console,
    alpha: f32,
}

impl DoryenRogue {
    pub fn new() -> Self {
        let mut hud = Console::new(HUD_WIDTH, HUD_HEIGHT);
        for y in 0..HUD_HEIGHT as i32 {
            for x in 0..HUD_WIDTH as i32 {
                hud.back(x, y, RED);
            }
        }
        hud.print((HUD_WIDTH / 2) as i32, 0, "SOME HUD", TextAlign::Center, None, None);

        Self {
            player: Player::new_player((0, 0)),
            entities: Vec::new(),
            mouse_pos: (0.0, 0.0),
            level: Level::new("src/level"),
            loaded: false,
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
        self.player.render(_api, &self.level);
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
            let mut mov = self.player.move_from_input(_api);
            if self.level.is_wall(self.player.next_pos((mov.0, 0))) {
                mov.0 = 0;
            }
            if self.level.is_wall(self.player.next_pos((0, mov.1))) {
                mov.1 = 0;
            }
            if self.player.move_by(mov,/* coef, */ _api) {
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
            self.level.render(_api, self.player.pos());
            self.render_entities(_api);
            let fps = _api.fps();
            _api.con().print_color(
                (CONSOLE_WIDTH / 2) as i32, 
                (CONSOLE_HIEGHT - 2) as i32, 
                &format!(
                    "#[white]Move with #[red]arrows or WASD #[white] {:4} fps", fps
                ), 
                TextAlign::Center,
                None,
            );
        } else {
            _api.con().print_color(
                (CONSOLE_WIDTH / 2) as i32, 
                (CONSOLE_HIEGHT / 2) as i32, 
                "#[white] Loading#[red]....", 
                TextAlign::Center, 
                None,
            );
        }
        self.hud.blit(
            (CONSOLE_WIDTH - HUD_WIDTH - 1) as i32, 
            (CONSOLE_HIEGHT - HUD_HEIGHT - 1) as i32, 
            _api.con(), 
            self.alpha, 
            self.alpha, 
            None);
    }
}
