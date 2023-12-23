use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.spawn((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 10,
        },
        FieldOfView::new(12),
        // Location(MapID::DevRoom01),
    ));
}
/*
example statue spawning

spawn_statue(&mut ecs, Point::new(8,8),"Abstract Statue".to_string() ,"A smooth statue with flowing curves".to_string() , "The statue is made out of a softly lavender stone polished down to a reflective finish that you can see a blurry mirror of your face in. Its form is undulating and surreal, looping back in on itself multiple times and sometimes splitting off into many fine strands that meld back into the main body. At the base of the statue there appears to be inscriptions in faded text. You can tell from the writing structure it's a poem, but in a dialect you don't quite understand.".to_string(), LocationID::FirstTown, Point3D::new(0, 0, 0));
*/

pub fn spawn_statue(
    ecs: &mut World,
    pos: Point,
    name: String,
    short_desc: String,
    long_desc: String,
    localmap: LocalMapID,
    map: Point3D,
) {
    ecs.spawn((
        pos,
        Render {
            color: ColorPair::new(LAVENDER, BLACK),
            glyph: to_cp437('δ'),
        },
        Name(name),
        ShortDescription(short_desc),
        LongDescription(long_desc),
        CurrentLocalMap(localmap),
        map,
    ));
}

pub fn spawn_monster(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    localmap: LocalMapID,
    pos_3d: Point3D,
    pos: Point,
) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.spawn((
        Enemy,
        CurrentLocalMap(localmap),
        pos,
        pos_3d,
        Render {
            color: ColorPair::new(YELLOW, BLACK),
            glyph,
        },
        ChasingPlayer {},
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        FieldOfView::new(6),
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

pub fn spawn_zombie(ecs: &mut World, localmap: LocalMapID, pos_3d: Point3D, pos: Point) {
    ecs.spawn((
        Enemy,
        Render {
            color: ColorPair::new(GREEN, BLACK),
            glyph: to_cp437('z'),
        },
        MovingRandomly{},
        Health {
            current: 5,
            max: 5,
        },
        Name("Zombie".to_string()),
        ShortDescription("A diseased human left to shamble the earth.".to_string()),
        LongDescription("Infected with some sort of unknown blight, what used to be a person has had their mind and body hollowed away. Somehow barely alive, they stumble blindly, reacting with instinctual aggression.".to_string()),
        CurrentLocalMap(localmap),
        pos_3d,
        pos,
        FieldOfView::new(4),
    ));
}
