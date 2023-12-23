use crate::prelude::*;

use super::library::{spawn_monster, spawn_zombie}; //reexamine this later it's best practice to minimize use of preludes
///function that will initialize all the entities in the first dungeon
pub fn init_first_dungeon(ecs: &mut World) {
    //filler all the enemies and various objects will go in here
    spawn_zombie(
        ecs,
        LocalMapID::FirstDungeon,
        Point3D::new(0, 0, 0),
        Point::new(3, 3),
    );
    let mut rng = RandomNumberGenerator::new();
    spawn_monster(
        ecs,
        &mut rng,
        LocalMapID::FirstDungeon,
        Point3D::new(0, 0, -1),
        Point::new(10, 11),
    );
    spawn_monster(
        ecs,
        &mut rng,
        LocalMapID::FirstDungeon,
        Point3D::new(0, 0, -1),
        Point::new(11, 11),
    );
    spawn_monster(
        ecs,
        &mut rng,
        LocalMapID::FirstDungeon,
        Point3D::new(0, 0, -1),
        Point::new(12, 12),
    );
    spawn_monster(
        ecs,
        &mut rng,
        LocalMapID::FirstDungeon,
        Point3D::new(0, 0, -1),
        Point::new(12, 11),
    );
    spawn_monster(
        ecs,
        &mut rng,
        LocalMapID::FirstDungeon,
        Point3D::new(0, 0, -1),
        Point::new(13, 13),
    );
}
