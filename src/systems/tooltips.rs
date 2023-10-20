use crate::prelude::*;

// #[system] //macro that does all the boilerplate to mark this function as a system
// #[read_component(Point)]
// #[read_component(Name)]
// #[read_component(FieldOfView)]
// #[read_component(Player)]
// #[read_component(Reticule)]
pub fn tooltips(state: &mut State) {
    let control_state = state.controlstate; //control state to decide what kind of tooltip is displayed
    let mut positions = state.ecs.query::<(&Point, &Name)>();
    let mut fov = state.ecs.query::<With<&FieldOfView, &Player>>();
    //gets the player's FOV so you can't use the tooltip to cheat and find monsters your PC can't see
    let mut reticule_pos = &Point::new(0, 0);

    if let Some((entity, pos)) = state.ecs.query::<With<&Point, &Reticule>>().iter().nth(0)
    //do a basic sanity check to make sure there even is a reticule in the world before you start doing logic with it
    {
        reticule_pos = state
            .ecs
            .query::<With<&Point, &Reticule>>()
            .iter()
            .nth(0)
            .unwrap()
            .1;
    }
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(TOOLTIP_LAYER);
    let player_fov = fov.iter().nth(0).unwrap().1;
    //this match statement decides what's displayed, whether it's an entity name, the chance to hit, etc.
    match control_state {
        ControlState::Default => {
            println!("This shouldn't happen! There should never be a reticule spawned in during the default controlstate!!");
        }
        ControlState::Looking => {
            positions
                .iter()
                .filter(|(_, (pos, _))| {
                    **pos == *reticule_pos && player_fov.visible_tiles.contains(&pos)
                })
                .for_each(|(entity, (_, name))| {
                    let screen_pos = *reticule_pos * 2;
                    let display = name.0.clone();

                    draw_batch.print(screen_pos, &display); //THIS WILL NEED TO BE TWEAKED.
                });
        }
    }

    draw_batch.submit(10100).expect("Batch error");
}
