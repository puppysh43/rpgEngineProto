use crate::prelude::*;

pub fn worldmap_render(state: &mut State) {
    if state.map_state == MapState::WorldMap {
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
        //render the player token
        let mut player_token_pos = Point::new(0, 0);
        for (_, pos) in state.ecs.query_mut::<With<&Point, &OverworldPlayerToken>>() {
            player_token_pos = *pos;
        }
        draw_batch.set(player_token_pos, ColorPair::new(CYAN, BLACK), to_cp437('@'));
        draw_batch.submit(5000).expect("Batch error");
    }
}
