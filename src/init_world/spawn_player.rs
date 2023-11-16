use crate::prelude::*;

pub fn spawn_player(ecs: &mut World) {
    //this will spawn in the player, which at a future point will include reading
    //off from a character sheet file generated before the gamestate is created
    //will also need to spawn the playertoken used for navigating the overworld

    //spawn player block
    ecs.spawn((
        Player,
        Render {
            color: ColorPair::new(PURPLE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 10,
        },
        FieldOfView::new(16),
    )); //gives the bare essentials, stuff like location or position will be added after the player
        //enters a worldmap tile in the overworld.

    //spawn player token for moving them around the overworld.
    let player_overworld_spawnpos = Point::new(0, 0);
    ecs.spawn((
        OverworldPlayerToken,
        // Render {
        // color: ColorPair::new(PURPLE, BLACK),
        // glyph: to_cp437('@'),
        // },
        player_overworld_spawnpos,
    ));
}
