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

spawn_statue(&mut ecs, Point::new(8,8),"Abstract Statue".to_string() ,"A smooth statue with flowing curves".to_string() , "The statue is made out of a softly lavender stone polished down to a reflective finish that you can see a blurry mirror of your face in. Its form is undulating and surreal, looping back in on itself multiple times and sometimes splitting off into many fine strands that meld back into the main body. At the base of the statue there appears to be inscriptions in faded text. You can tell from the writing structure it's a poem, but in a dialect you don't quite understand.".to_string(), MapID::DevRoom01);
*/

pub fn spawn_statue(
    ecs: &mut World,
    pos: Point,
    name: String,
    short_desc: String,
    long_desc: String,
    location: LocationID,
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
        CurrentLocation(location),
        map,
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.spawn((
        Enemy,
        pos,
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
