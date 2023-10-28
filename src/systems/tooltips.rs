use crate::prelude::*;

pub fn tooltips(state: &mut State) {
    //TODO will need to filter the positions query to include only things sharing the same location as the player
    let control_state = state.controlstate; //control state to decide what kind of tooltip is displayed
    if state.uistate == UiState::Default {
        let mut positions = state.ecs.query::<(&Point, &Name)>();
        let mut fov = state.ecs.query::<With<&FieldOfView, &Player>>();
        //gets the player's FOV so you can't use the tooltip to cheat and find monsters your PC can't see
        let mut reticule_pos = Point::new(0, 0);
        for (_, pos) in state.ecs.query::<With<&Point, &Reticule>>().iter() {
            reticule_pos = pos.clone();
        }

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(TOOLTIP_LAYER);
        let player_fov = fov.iter().nth(0).unwrap().1;
        //this match statement decides what's displayed, whether it's an entity name, the chance to hit, etc.
        match control_state {
            ControlState::Looking => {
                positions
                    .iter()
                    .filter(|(_, (pos, _))| {
                        **pos == reticule_pos && player_fov.visible_tiles.contains(&pos)
                    })
                    .for_each(|(entity, (_, name))| {
                        let screen_pos = reticule_pos * 2;
                        let display = name.0.clone();

                        draw_batch.print(screen_pos, &display); //THIS WILL NEED TO BE TWEAKED.
                    });
            }
            _ => {
                //filler comment because we don't need to draw a tooltip!
            }
        }

        draw_batch.submit(10100).expect("Batch error");
    }
}
