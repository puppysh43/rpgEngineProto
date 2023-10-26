use crate::prelude::*;
///Function used for general printf debugging so I can keep it from contaminating my normal functions
pub fn println_debugger(state: &mut State) {
  //this block will check for a reticule and print out the position and if it exists
  for (reticule_entity, pos) in state.ecs.query::<With<&Point, &Reticule>>().iter() {
      println!("There is a reticule in the ECS and its coordinates are x:{} y:{}", pos.x, pos.y);
  }
}
