#![warn(clippy::pedantic)]
//I'm gonna switch this over to HECS!!
mod components;
mod control_state;
mod init_world;
mod location;
mod map;
mod mapstate;
mod overworld;
mod systems;
mod turn_state;
mod ui_state;
mod worldgen;

mod prelude {
    //libraries used
    pub use bracket_lib::prelude::*;
    pub use hecs::*;
    pub use std::collections::BTreeMap;
    pub use std::collections::HashMap;
    //size of the full window including UI elements
    pub const SCREEN_WIDTH: i32 = 70;
    pub const SCREEN_HEIGHT: i32 = 35;
    //size of the ingame map and play area, the rest is reserved for UI, menus, log, etc
    pub const MAP_WIDTH: i32 = 50;
    pub const MAP_HEIGHT: i32 = 35;
    pub const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;
    //UI sizing information

    pub const UI_XSTART: i32 = (MAP_WIDTH * 2) + 1;
    pub const UI_BORDER_X: i32 = MAP_WIDTH * 2;
    pub const LOG_YSTART: i32 = 11;

    ///virtual terminal where the main game is displayed - map, entities, etc    
    pub const MAIN_LAYER: usize = 0;
    ///virtual terminal used for effects in the map area such as particles, gas, UI elements such as reticules, etc
    pub const EFFECTS_LAYER: usize = 1;
    ///virtual terminal used for tooltips using the 8x8 terminal font such as entity names, hitchance when targeting enemies, etc
    pub const TOOLTIP_LAYER: usize = 2;
    ///virtual terminal used for the text bar on the side of the screen w/ the log, stats, etc
    pub const UI_LAYER: usize = 3;

    pub use crate::components::ai_components::*;
    pub use crate::components::messages_of_intent::*;
    pub use crate::components::*;
    pub use crate::control_state::*;
    pub use crate::location::*;
    pub use crate::map::*;
    pub use crate::mapstate::*;
    pub use crate::overworld::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
    pub use crate::ui_state::*;
    pub use crate::worldgen::gen_locations::NUM_LOCATIONS;
    pub use crate::State;
}

// use init_world;
use prelude::*;
use worldgen::gen_locations;
use worldgen::gen_overworld;
pub struct State {
    ecs: World,                  //our entity component system
    key: Option<VirtualKeyCode>, //the current key detected as being press
    shift: bool,
    control: bool,
    alt: bool,
    //will later need to add things such as if shift and control is being pressed
    //or even mouse information
    turnstate: TurnState,       //this controls the flow of our turn-based game
    controlstate: ControlState, //keeps track of what the player is doing to decide what keys do what
    locations: Locations,       //all of the localmaps used to store world data
    worldmap: WorldMap,
    player: Entity,
    log: Vec<String>,
    numberturns: u32, //each turn represents 1 second
    uistate: UiState, //used to track what menu the player is in
    map_state: MapState,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::new();
        let worldmap = gen_overworld::generate_overworld(); //generate the worldmap
        let locations = gen_locations::generate_locations(); //generate the game's locations
        let log: Vec<String> = Vec::new(); //generate a blank log
        init_world::init_world(&mut ecs); //pass the ecs to this and it will spawn all the entities needed in the gameworld.

        let player = ecs.query::<&Player>().iter().nth(0).unwrap().0;
        Self {
            ecs,
            key: None,
            shift: false,
            control: false,
            alt: false,
            turnstate: TurnState::AwaitingInput,
            controlstate: ControlState::InOverworld,
            locations,
            worldmap,
            player,
            log,
            numberturns: 0,
            uistate: UiState::Default,
            map_state: MapState::WorldMap, //temporary for now probably best if players start in a town or something first
        }
    }
    /*
    temporarily disabled
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
    }*/
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
        self.shift = ctx.shift;
        self.control = ctx.control;
        self.alt = ctx.alt;
        //will maybe need to add line to get whether or not the player is pressing shift or control to allow for more nuanced controls
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

    main_loop(context, State::new())
}
