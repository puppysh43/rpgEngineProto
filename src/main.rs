#![warn(clippy::pedantic)]

mod components;
mod control_state;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    //size of the full window including UI elements
    pub const SCREEN_WIDTH: i32 = 70;
    pub const SCREEN_HEIGHT: i32 = 35;
    //size of the ingame map and play area, the rest is reserved for UI, menus, log, etc
    pub const MAP_WIDTH: i32 = 50;
    pub const MAP_HEIGHT: i32 = 35;

    ///virtual terminal where the main game is displayed - map, entities, etc    
    pub const MAIN_LAYER: usize = 0;
    ///virtual terminal used for effects in the map area such as particles, gas, UI elements such as reticules, etc
    pub const EFFECTS_LAYER: usize = 1;
    ///virtual terminal used for tooltips using the 8x8 terminal font such as entity names, hitchance when targeting enemies, etc
    pub const TOOLTIP_LAYER: usize = 2;
    ///virtual terminal used for the text bar on the side of the screen w/ the log, stats, etc
    pub const UI_LAYER: usize = 3;

    pub use crate::components::*;
    pub use crate::control_state::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));
        resources.insert(map_builder.map);
        resources.insert(TurnState::AwaitingInput);
        resources.insert(ControlState::Default);
        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut self.ecs, map_builder.player_start);
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut self.ecs, &mut rng, pos));
        self.resources.insert(map_builder.map);
        self.resources.insert(TurnState::AwaitingInput);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come to a premature end.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Amulet of Yala remains unclaimed, and your home town is not saved.",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't worry, you can always try again with a new hero.",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(MAIN_LAYER); //set the active console to the first virtual terminal used for map tiles & entities
        ctx.cls(); //clear it to prevent bleedover of printed tiles between ticks
        ctx.set_active_console(EFFECTS_LAYER); //set the active s
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        ctx.set_active_console(3);
        ctx.cls();
        self.resources.insert(ctx.key); //give the ECS access to what keys are being pressed at the time of the tick
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos())); //gives the ecs access to the current mouse position
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => {
                self.player_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => {
                self.game_over(ctx);
            }
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("RPG Engine")
        .with_fps_cap(30.0)
        .with_fitscreen(false)
        .with_fullscreen(false)
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        // .with_dimensions(MAP_WIDTH, MAP_HEIGHT)
        .with_tile_dimensions(24, 24)
        .with_resource_path("resources/")
        .with_font("main_font.png", 24, 24)
        .with_font("effects_font.png", 24, 24)
        .with_font("tooltip_font.png", 12, 12)
        .with_font("ui_font.png", 12, 24)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "main_font.png") //console that the map prints to
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "effects_font.png") //console for effects
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "tooltip_font.png") //console for text popups in game
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT, "ui_font.png") //console for text on the sidebar for the UI and logs
        .build()?;
    //in the future both entities and map tiles will be on the same terminal, with one for UI elements, one 12x24 for the log/UI and the 8x8 for text popups

    main_loop(context, State::new())
}
