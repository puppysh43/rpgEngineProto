#![warn(clippy::pedantic)]
//I'm gonna switch this over to HECS!!
mod components;
mod control_state;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;
mod ui_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use hecs::*;
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
    pub use crate::ui_state::*;
    pub use crate::State;
}

use prelude::*;

pub struct State {
    ecs: World,
    key: Option<VirtualKeyCode>,
    turnstate: TurnState,
    controlstate: ControlState,
    map: Map,
    log: Vec<String>,
    numberturns: u32, //each turn represents 1 second
    uistate: UiState,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::new();
        let map = build_devroom01();
        let log: Vec<String> = Vec::new();
        spawn_player(&mut ecs, Point::new(1, 1)); //needs to be updated
        spawn_statue(&mut ecs, Point::new(8,8),"Abstract Statue".to_string() ,"A smooth statue with flowing curves".to_string() , "The statue is made out of a softly lavender stone polished down to a reflective finish that you can see a blurry mirror of your face in. Its form is undulating and surreal, looping back in on itself multiple times and sometimes splitting off into many fine strands that meld back into the main body.".to_string());
        Self {
            ecs,
            key: None,
            turnstate: TurnState::AwaitingInput,
            controlstate: ControlState::Default,
            map,
            log,
            numberturns: 0,
            uistate: UiState::Default,
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.key = None;
        let map = build_devroom01();
        self.map = map;
        self.turnstate = TurnState::AwaitingInput;
        self.controlstate = ControlState::Default;
        self.log = vec!["Welcome to my game!".to_string()];
        self.numberturns = 0;
        self.uistate = UiState::Default;
        spawn_player(&mut self.ecs, Point::new(1, 1)); //placeholder position the spawn player function will be handled by the
                                                       //then I'll need to reset all the entity spawns lmao
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(MAIN_LAYER); //set the active console to the first virtual terminal used for map tiles & entities
        ctx.cls(); //clear it to prevent bleedover of printed tiles between ticks
        ctx.set_active_console(EFFECTS_LAYER); //set the active s
        ctx.cls();
        ctx.set_active_console(TOOLTIP_LAYER);
        ctx.cls();
        ctx.set_active_console(UI_LAYER);
        ctx.cls();
        self.key = ctx.key;
        systems::run_systems(self);
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
