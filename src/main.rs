#![warn(clippy::pedantic)]
mod components;
mod control_state;
mod gfx_config;
mod init_world;
mod interactionmenu;
mod localmap;
mod mapscreen;
mod mapstate;
mod systems;
mod turn_state;
mod ui_state;
mod worldgen;
mod worldmap;

mod prelude {
    //libraries used
    pub use bracket_lib::prelude::*;
    pub use hecs::*;
    pub use serde::*;
    pub use serde_json::*;
    pub use std::collections::HashMap;
    //size of the full window including UI elements
    pub const SCREEN_WIDTH: i32 = 120;
    pub const SCREEN_HEIGHT: i32 = 40;
    //size of the ingame map and play area, the rest is reserved for UI, menus, log, etc
    pub const MAP_WIDTH: i32 = 70;
    pub const MAP_HEIGHT: i32 = 40;
    pub const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;
    //UI sizing information

    pub const UI_XSTART: i32 = (MAP_WIDTH) + 1;
    pub const UI_BORDER_X: i32 = MAP_WIDTH;
    pub const LOG_YSTART: i32 = 11;

    ///virtual terminal where the main game is displayed - map, entities, etc    
    pub const MAIN_LAYER: usize = 0;
    ///virtual terminal used for effects in the map area such as particles, gas, UI elements such as reticules, etc
    pub const EFFECTS_LAYER: usize = 1;
    ///virtual terminal used for tooltips using the 8x8 terminal font such as entity names, hitchance when targeting enemies, etc
    pub const TOOLTIP_LAYER: usize = 2;
    ///virtual terminal used for the text bar on the side of the screen w/ the log, stats, etc
    pub const UI_LAYER: usize = 3;

    //import constants defining the graphics
    pub use crate::gfx_config::pallete::*;
    pub use crate::gfx_config::tileset::*;

    pub use crate::components::ai_components::*;
    pub use crate::components::items::*;
    pub use crate::components::messages_of_intent::*;
    pub use crate::components::rpg_systems::*;
    pub use crate::components::*;
    pub use crate::control_state::*;
    pub use crate::interactionmenu::*;
    pub use crate::localmap::*;
    pub use crate::mapscreen::*;
    pub use crate::mapstate::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
    pub use crate::ui_state::*;
    pub use crate::worldgen::gen_localmaps::NUM_LOCALMAPS;
    pub use crate::worldmap::*;
    pub use crate::State;
}

// use init_world;
use prelude::*;
use worldgen::gen_localmaps;
use worldgen::gen_worldmap;
///This struct is just a giant singleton that holds all relevant gamestate info - our ECS, non-ECS gamedata,
///as well as input information about what keys are being pressed.
pub struct State {
    ecs: World,                  //our entity component system
    key: Option<VirtualKeyCode>, //the current key detected as being press
    shift: bool,
    control: bool,
    alt: bool,
    turnstate: TurnState,       //this controls the flow of our turn-based game
    controlstate: ControlState, //keeps track of what the player is doing to decide what keys do what
    localmaps: LocalMaps,       //all of the localmaps used to store world data
    worldmap: WorldMap,
    player: Entity,
    log: Vec<String>,
    numberturns: u32,    //each turn represents 1 second
    uistate: UiState,    //used to track what menu the player is in
    map_state: MapState, //tracks if the player is in the worldmap or localmap
    int_menu_db: InteractionMenuDatabase,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::new();
        let worldmap = gen_worldmap::generate_worldmap(); //generate the worldmap
        let localmaps = gen_localmaps::generate_localmaps(); //generate the game's locations
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
            controlstate: ControlState::InWorldMap,
            localmaps,
            worldmap,
            player,
            log,
            numberturns: 0,
            uistate: UiState::Default,
            map_state: MapState::WorldMap, //temporary for now probably best if players start in a town or something first
            int_menu_db: init_int_menu_db(),
        }
    }

    fn reset_gamestate(&mut self) {
        //this will reset the gamestate! I'll need this later
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(MAIN_LAYER); //set the active console to the first virtual terminal used for map tiles & entities
        ctx.cls(); //clear it to prevent bleedover of printed tiles between ticks
        ctx.set_active_console(EFFECTS_LAYER);
        ctx.cls();
        ctx.set_active_console(TOOLTIP_LAYER);
        ctx.cls();
        ctx.set_active_console(UI_LAYER);
        ctx.cls();
        //get player input
        self.key = ctx.key;
        self.shift = ctx.shift;
        self.control = ctx.control;
        self.alt = ctx.alt;
        //run all the tick systems
        systems::run_systems(self);
        //render all the draw buffers generated by the systems
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
        .with_tile_dimensions(16, 24)
        .with_resource_path("resources/")
        .with_font("main_font.png", 16, 24)
        .with_font("effects_font.png", 16, 24)
        .with_font("tooltip_font.png", 16, 24)
        .with_font("ui_font.png", 16, 24)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "main_font.png") //console that the map prints to
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "effects_font.png") //console for effects
        // .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "tooltip_font.png") //console for text popups in game
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "tooltip_font.png") //console for text popups in game
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "ui_font.png") //console for text on the sidebar for the UI and logs
        .build()?;
    // context.with_post_scanlines(true);

    main_loop(context, State::new())
}

pub fn init_int_menu_db() -> InteractionMenuDatabase {
    let mut int_menu_db_contents: HashMap<String, InteractionMenu> = HashMap::new();
    let mut boulder = InteractionMenu::new_blank("A large boulder lies in front of you, blocking the tunnel you need to pass through to reach your goal. It looks immensely heavy, a craggy face of granite only the strongest adventurers could move.".to_string());
    let examine_boulder = IntMenuChoice::new(
        "Examine The Boulder".to_string(),
        None,
        None,
        ResultText::new_static_result_text("Despite its invincible exterior, the boulder is spiderwebbed with countless hairline fractures suggesting it could easily be broken apart with explosives or digging equipment.".to_string())
    );
    boulder.add_entry(examine_boulder);
    let move_boulder = IntMenuChoice::new("[Athletics Check] Try and push the boulder aside.".to_string(), None,
    Some(move_boulder_skillcheck), ResultText::new_deg_of_success_result_text("You fail to move the boulder at all, and are left only sweaty and out of breath for your effort".to_string(),
            "You're able to rock the boulder an an axis just enough to rotate it in the hallway, shifting it enough to leave a small crack. You can probably fit, but it might hurt.".to_string() , 
        "In a feat of herculean strength you're able to completely push the boulder away from the entrance of the tunnel, clearing your way.".to_string()));
    boulder.add_entry(move_boulder);
    int_menu_db_contents.insert("Boulder".to_string(), boulder);
    InteractionMenuDatabase::new(int_menu_db_contents)
}
fn move_boulder_skillcheck(
    state: &mut State,
    commands: &mut CommandBuffer,
) -> Option<ChoiceResult> {
    let player_skills = get_player_skills(state);
    //moving a boulder is a big task, have it a -1 athletics skillcheck
    let check_result = player_skills.skillcheck(SkillType::Athletics, -1);
    match check_result {
        RollResult::Failure => {
            //if this were the real game we would subtract some fatigue points or something from the player
            return Some(ChoiceResult::DegOfSuccess(DegreeOfSuccess::Failure));
        }
        RollResult::PartialSuccess => {
            //in a real game this would switch a flag in the state that was hiding another
            //option in the menu to let the player squeeze through in exchange for taking
            //damage if they fail a reflexes/dexterity check
            return Some(ChoiceResult::DegOfSuccess(DegreeOfSuccess::PartialSuccess));
        }
        RollResult::FullSuccess => {
            //in the real game this would go into the state and delete the position component
            //of the boulder so that it would dissapear from the map and no longer block collissions
            return Some(ChoiceResult::DegOfSuccess(DegreeOfSuccess::FullSuccess));
        }
    }
}

fn get_player_skills(state: &State) -> Skills {
    let player = state.player.clone();
    let player_skills = state
        .ecs
        .query_one::<&Skills>(player)
        .expect("Player doesn't have a skills component.")
        .get()
        .unwrap()
        .clone();
    player_skills
}
