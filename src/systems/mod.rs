use crate::prelude::*;

mod chasing;
mod combat;
mod effects_render;
mod end_turn;
mod entity_render;
mod fov;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod tooltips;
mod ui_render;
mod update_log;

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
    player_input::player_input(state);
    fov::fov(state);
    update_log::update_log(state);
    map_render::map_render(state);
    entity_render::entity_render(state);
    effects_render::effects_render(state);
    tooltips::tooltips(state);
    ui_render::ui_render(state);
}
///All player related functions go here.
fn pc_systems(state: &mut State) {
    combat::combat(state); //will need to tweak the combat system
    movement::movement(state);
    fov::fov(state);
    update_log::update_log(state);
    map_render::map_render(state);
    entity_render::entity_render(state);
    effects_render::effects_render(state);
    ui_render::ui_render(state);
    end_turn::end_turn(state);
}
///All NPC related systems as well as worldsystems that progress once a turn such as
///the spread of fire, growth of plants, etc.
fn npc_systems(state: &mut State) {
    random_move::random_move(state);
    chasing::chasing(state);
    combat::combat(state);
    movement::movement(state);
    fov::fov(state);
    update_log::update_log(state);
    map_render::map_render(state);
    entity_render::entity_render(state);
    effects_render::effects_render(state);
    ui_render::ui_render(state);
    end_turn::end_turn(stat);
}
///will play a screen telling the player they died, maybe show some stats, and then
///will ask if they either wanna reload or check
fn game_over(state: &mut State) {
    //filler
}

///plays through ending slides
fn ending_slides(state: &mut State) {
    //filler
}

///the main menu, used for saving and loading games
fn main_menu(state: &mut State) {
    //filler
}
