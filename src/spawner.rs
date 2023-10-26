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
    ));
}

pub fn spawn_statue(
    ecs: &mut World,
    pos: Point,
    name: String,
    short_desc: String,
    long_desc: String,
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
