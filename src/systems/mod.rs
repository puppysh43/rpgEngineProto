use crate::prelude::*;

mod ai;
mod combat;
mod debugging;
mod effects_render;
mod end_turn;
mod entity_render;
mod fov;
mod get_player_location;
mod map_render;
mod map_transition;
mod movement;
mod player_input;
mod tooltips;
mod ui_render;
mod update_log;
//TODO IMPLEMENT A FUCKING FLUSH SYSTEM AND PERSISTENT

pub fn run_systems(state: &mut State) {
    let current_turn = state.turnstate;
    match current_turn {
        TurnState::MainMenu => main_menu(state),
        TurnState::AwaitingInput => input_systems(state),
        TurnState::PcTurn => pc_systems(state),
        TurnState::NpcTurn => npc_systems(state),
        TurnState::GameOver => game_over(state),
        TurnState::EndingSlides => ending_slides(state),
    }
}

///All input related systems that happen before a player's turn is over such as looking,
///talking to NPCs, going through their inventory, etc.
fn input_systems(state: &mut State) {
    let mut commands = CommandBuffer::new();
    get_player_location::get_player_location(state);
    player_input::player_input(state, &mut commands); //need to update this to work w/ the new map system
    commands.run_on(&mut state.ecs);
    fov::fov(state, &mut commands); //done I think? Will need to doublecheck
    commands.run_on(&mut state.ecs);
    update_log::update_log(state, &mut commands); //WORKING(?)
    commands.run_on(&mut state.ecs);
    map_render::map_render(state); //WORKING(?)
    entity_render::entity_render(state); //WORKING(?)
    effects_render::effects_render(state); //WORKING(?)
    tooltips::tooltips(state); //needs to be updated to work w/ multiple locations plus tweak text
    ui_render::ui_render(state, &mut commands); //WORKING(?)
    commands.run_on(&mut state.ecs);
    // map_transition::map_transitions(state, &mut commands);
    // commands.run_on(&mut state.ecs);
    debugging::println_debugger(state);
}
///All player related functions go here.
fn pc_systems(state: &mut State) {
    let mut commands = CommandBuffer::new();
    // get_player_location::get_player_location(state);
    combat::combat(state, &mut commands); //
    commands.run_on(&mut state.ecs);
    movement::movement(state, &mut commands); // WORKING (????)
    commands.run_on(&mut state.ecs);
    fov::fov(state, &mut commands); //WORKING(?)
    commands.run_on(&mut state.ecs);
    update_log::update_log(state, &mut commands); //WORKING(?)
    commands.run_on(&mut state.ecs);
    map_render::map_render(state); //WORKING(?)
    entity_render::entity_render(state); //WORKING(?)
    effects_render::effects_render(state); //WORKING(?)
    ui_render::ui_render(state, &mut commands); //WORKING(?)
    commands.run_on(&mut state.ecs);
    map_transition::map_transitions(state, &mut commands);
    commands.run_on(&mut state.ecs);
    end_turn::end_turn(state); //WORKING(?)
}
///All NPC related systems as well as worldsystems that progress once a turn such as
///the spread of fire, growth of plants, etc.
fn npc_systems(state: &mut State) {
    let mut commands = CommandBuffer::new();
    get_player_location::get_player_location(state);
    ai::ai_systems(state, &mut commands);
    combat::combat(state, &mut commands); //WORKING (????)
    commands.run_on(&mut state.ecs);
    movement::movement(state, &mut commands); //WORKING (????)
    commands.run_on(&mut state.ecs);
    fov::fov(state, &mut commands); //WORKING(?)
    commands.run_on(&mut state.ecs);
    update_log::update_log(state, &mut commands); //WORKING(?)
    commands.run_on(&mut state.ecs);
    map_render::map_render(state); //WORKING(?)
    entity_render::entity_render(state); //WORKING(?)
    effects_render::effects_render(state); //WORKING(?)
    ui_render::ui_render(state, &mut commands); //WORKING(?)
    commands.run_on(&mut state.ecs);

    // debugging::println_debugger(state);
    end_turn::end_turn(state); //WORKING(?)
}
///will play a screen telling the player they died, maybe show some stats, and then
///will ask if they either wanna reload or check
fn game_over(state: &mut State) {
    println!("This is the game over state");
    //filler
    //this will ask the player if they want to quit back to the main menu to load their savegame/start a new game or if they want to watch the ending slides
    //given what happened
}

///plays through ending slides
fn ending_slides(state: &mut State) {
    //filler
}

///the main menu, used for saving and loading games
fn main_menu(state: &mut State) {
    //filler
}
