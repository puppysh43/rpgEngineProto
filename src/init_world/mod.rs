//this file will be used to have a single "init world" function that will subdelegate things to their own specialized functions
//then use the library of spawner functions for all the various regions
//will need to spawn the player as its own function b/c I'll later need to add functionality to
use crate::prelude::*; //reconsider this/limit the amount that have access to preludes.
mod init_first_dungeon;
mod init_first_town;
mod library; //maybe don't need this one
mod spawn_player;

// use init_first_dungeon::*;
// use init_first_town::*;
// use spawn_player::*;

pub fn init_world(ecs: &mut World) {
    //filler
    //will need function to spawn in the player as well as the player token
    spawn_player::spawn_player(ecs);
    //will need function to spawn in all the entities in the starting town
    init_first_town::init_first_town(ecs);
    //will need a function to spawn in all the entities of the first dungeon
    init_first_dungeon::init_first_dungeon(ecs);
}
