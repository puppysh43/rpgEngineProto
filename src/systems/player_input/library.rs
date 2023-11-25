use crate::prelude::*;

pub fn spawn_reticule(cmd: &mut CommandBuffer, player_pos: Point) {
    cmd.spawn((
        //creates a reticule object in the world
        Effect,
        Reticule,
        player_pos,
        Render {
            color: ColorPair::new(CYAN, BLACK),
            glyph: to_cp437('♥'),
        },
    ));
}
