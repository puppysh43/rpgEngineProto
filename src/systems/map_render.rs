use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(#[resource] map: &Map, ecs: &SubWorld) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    let player_fov = fov.iter(ecs).nth(0).unwrap();

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let pt = Point::new(x, y);
            let idx = map_idx(x, y);
            if map.in_bounds(pt)
                && (player_fov.visible_tiles.contains(&pt) | map.revealed_tiles[idx])
            {
                // (1)
                let tint = if player_fov.visible_tiles.contains(&pt) {
                    // (2)
                    WHITE
                } else {
                    DARK_GRAY
                };
                match map.tiles[idx] {
                    TileType::Floor => {
                        draw_batch.set(
                            pt,
                            ColorPair::new(
                                tint, // (3)
                                BLACK,
                            ),
                            to_cp437('.'),
                        );
                    }
                    TileType::Wall => {
                        draw_batch.set(pt, ColorPair::new(tint, BLACK), to_cp437('#'));
                    }
                }
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
