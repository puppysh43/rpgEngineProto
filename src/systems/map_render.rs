use crate::prelude::*;

pub fn map_render(state: &mut State) {
    let mut fov = state.ecs.query::<With<&FieldOfView, &Player>>();
    let mut draw_batch = DrawBatch::new();
    let mut map = Map::new();
    for (player, raw_loc) in state.ecs.query::<With<&Location, &Player>>().iter() {
        map = state
            .localmaps
            .get(&raw_loc.0)
            .expect("Player's MapID is not valid.")
            .clone();
    }

    draw_batch.target(0);

    let player_fov = fov.iter().nth(0).unwrap().1; //extract the player's FOV

    if state.uistate == UiState::Default {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let pt = Point::new(x, y);
                let idx = map_idx(x, y);
                if map.in_bounds(pt)
                    && (player_fov.visible_tiles.contains(&pt) | map.revealed_tiles[idx])
                {
                    let tint = if player_fov.visible_tiles.contains(&pt) {
                        // will need to switch this over to darkening various colours in the pallete.
                        WHITE
                    } else {
                        DARK_GRAY
                    };
                    match map.tiles[idx] {
                        TileType::Floor => {
                            draw_batch.set(pt, ColorPair::new(tint, BLACK), to_cp437('.'));
                        }
                        TileType::Wall => {
                            draw_batch.set(pt, ColorPair::new(tint, BLACK), to_cp437('#'));
                        }
                        TileType::MapPortal(MapID) => {
                            draw_batch.set(pt, ColorPair::new(PINK, BLACK), to_cp437('*'));
                        }
                    }
                }
            }
        }
        draw_batch.submit(0).expect("Batch error");
    }
}
