use crate::prelude::*;

pub fn map_render(state: &mut State) {
    if !state.is_in_overworld {
        //don't do this unless the player isn't in the overworld
        let mut fov = state.ecs.query::<With<&FieldOfView, &Player>>();
        let mut draw_batch = DrawBatch::new();

        let mut player_location = LocationID::FirstTown; //temp variable to be overwritten
        let mut player_coords = Point3D::new(0, 0, 0);
        for (_, (current_location, coords)) in state
            .ecs
            .query::<With<(&CurrentLocation, &Point3D), &Player>>()
            .iter()
        {
            player_location = current_location.0;
            player_coords = *coords;
        }

        let map = state
            .locations
            .get(&player_location)
            .expect("Player's MapID is not valid.")
            .get_map(player_coords)
            .clone();

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
                            TileType::StairUp => {
                                draw_batch.set(pt, ColorPair::new(tint, BLACK), to_cp437('<'));
                            }
                            TileType::StairDown => {
                                draw_batch.set(pt, ColorPair::new(tint, BLACK), to_cp437('>'));
                            }
                            TileType::MapTransitionNorth => {
                                draw_batch.set(pt, ColorPair::new(GREEN, BLACK), to_cp437('↑'));
                            }
                            TileType::MapTransitionEast => {
                                draw_batch.set(pt, ColorPair::new(GREEN, BLACK), to_cp437('→'));
                            }
                            TileType::MapTransitionSouth => {
                                draw_batch.set(pt, ColorPair::new(GREEN, BLACK), to_cp437('↓'));
                            }
                            TileType::MapTransitionWest => {
                                draw_batch.set(pt, ColorPair::new(GREEN, BLACK), to_cp437('←'));
                            }
                        }
                    }
                }
            }
            draw_batch.submit(0).expect("Batch error");
        }
    }
}
