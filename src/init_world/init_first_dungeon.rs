use crate::prelude::*;

use super::library::spawn_zombie; //reexamine this later it's best practice to minimize use of preludes
///function that will initialize all the entities in the first dungeon
pub fn init_first_dungeon(ecs: &mut World) {
    //filler all the enemies and various objects will go in here
    spawn_zombie(
        ecs,
        LocationID::FirstDungeon,
        Point3D::new(0, 0, 0),
        Point::new(3, 3),
    );
}
