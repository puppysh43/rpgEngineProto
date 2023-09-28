use crate::prelude::*;

#[system] //macro that does all the boilerplate to mark this function as a system
#[read_component(Point)]
#[read_component(Name)]
#[read_component(FieldOfView)]
#[read_component(Player)]
#[read_component(Reticule)]
pub fn tooltips(ecs: &SubWorld, #[resource] control_state: &ControlState) {
    //function requests access to the current control state so that it can know what to display
    let mut positions = <(Entity, &Point, &Name)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>()); //gets the player's FOV so you can't use the tooltip to cheat and find monsters your PC can't see
    let mut reticule_pos = &Point::new(0, 0);
    if let Some(&pos) = <&Point>::query()
        .filter(component::<Reticule>())
        .iter(ecs)
        .nth(0)
    {
        reticule_pos = <&Point>::query()
            .filter(component::<Reticule>())
            .iter(ecs)
            .nth(0)
            .unwrap();
    }
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(TOOLTIP_LAYER);
    let player_fov = fov.iter(ecs).nth(0).unwrap();
    match control_state {
        ControlState::Default => {
            positions
                .iter(ecs)
                .filter(|(_, pos, _)| {
                    **pos == *reticule_pos && player_fov.visible_tiles.contains(&pos)
                })
                .for_each(|(entity, _, name)| {
                    let screen_pos = *reticule_pos * 2;
                    let display = if let Ok(health) =
                        ecs.entry_ref(*entity).unwrap().get_component::<Health>()
                    {
                        format!("{} : {} hp", &name.0, health.current)
                    } else {
                        name.0.clone()
                    };
                    draw_batch.print(screen_pos, &display);
                });
        }
        ControlState::Looking => {
            positions
                .iter(ecs)
                .filter(|(_, pos, _)| {
                    **pos == *reticule_pos && player_fov.visible_tiles.contains(&pos)
                })
                .for_each(|(entity, _, name)| {
                    let screen_pos = *reticule_pos * 2;
                    let display = if let Ok(health) =
                        ecs.entry_ref(*entity).unwrap().get_component::<Health>()
                    {
                        format!("{} : {} hp", &name.0, health.current)
                    } else {
                        name.0.clone()
                    };
                    draw_batch.print(screen_pos, &display);
                });
        }
    }

    draw_batch.submit(10100).expect("Batch error");
    //have a match statement that decides whether it displays the name of the entity, the chance to hit with a ranged attack, etc
}
