use crate::prelude::*;

pub fn overworld_render(state: &mut State) {
    if state.is_in_overworld {
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(0);
        let worldmap = &state.worldmap;
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let pt = Point::new(x, y);
                let idx = map_idx(x, y);
                if worldmap.in_bounds(pt) {
                    match worldmap.tiles[idx] {
                        WorldTileType::Town(_) => {
                            draw_batch.set(pt, ColorPair::new(YELLOW, BLACK), to_cp437('⌂'));
                        }
                        WorldTileType::Dungeon(_) => {
                            draw_batch.set(pt, ColorPair::new(GREEN, BLACK), to_cp437('‼'));
                        }
                        WorldTileType::Desert => {
                            draw_batch.set(pt, ColorPair::new(DARKGRAY, LIGHTGRAY), to_cp437('≈'));
                        }
                    }
                }
            }
        }
        //TODO render the player token
    }
}
