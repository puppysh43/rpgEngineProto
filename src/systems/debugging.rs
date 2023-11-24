use crate::prelude::*;
///Function used for general printf debugging so I can keep it from contaminating my normal functions
pub fn println_debugger(state: &mut State) {
    //this block will check for a reticule and print out the position and if it exists
    // for (_, pos) in state.ecs.query::<With<&Point, &Reticule>>().iter() {
    // println!(
    // "There is a reticule in the ECS and its coordinates are x:{} y:{}",
    // pos.x, pos.y
    // );
    // }

    /*
    for (_, pos) in state
        .ecs
        .query::<With<&Point, &OverworldPlayerToken>>()
        .iter()
    {
        println!(
            "The player's overworld token position is x:{} y:{}",
            pos.x, pos.y
        );
    }
    for (_, location) in state.ecs.query_mut::<With<&CurrentLocation, &Player>>() {
        println!("The current location of the player is {:?}", location.0);
    }*/
    /*
    for (_, (pos, pos_3d, location)) in state
        .ecs
        .query_mut::<With<(&Point, &Point3D, &CurrentLocation), &Player>>()
    {
        println!("The player's position on the mapscreen is x: {}, y: {} \n the player's 3D position in the location is x:{}, y:{}, z:{}", pos.x, pos.y, pos_3d.x, pos_3d.y, pos_3d.z);
        match location.0 {
            LocationID::FirstTown => println!("Player's current location is the first town."),
            LocationID::FirstDungeon => println!("Player's current location is the first dungeon."),
        }
    }*/
    // println!("Current UI State is: {:?}", state.uistate);
}
