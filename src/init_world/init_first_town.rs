use crate::init_world::library::*;
use crate::prelude::*; //reconsider this later may not be necessary! in general its best to limit what has prelude access
pub fn init_first_town(ecs: &mut World) {
    spawn_statue(
        ecs,
        Point::new(8,8),
        "Abstract Statue".to_string(),
        "A smooth statue with flowing curves and veined with extravagant colors.".to_string(),
        "The statue is made out of a softly lavender stone polished down to a reflective finish that you can see a blurry mirror of your face in. Its form is undulating and surreal, looping back in on itself multiple times and sometimes splitting off into many fine strands that meld back into the main body. At the base of the statue there appears to be inscriptions in faded text. You can tell from the writing structure it's a poem, but in a dialect you don't quite understand.".to_string(),
        LocationID::FirstTown,
        Point3D::new(0, 0, 0)
    );
    spawn_statue(ecs,
        Point::new(20, 10),
        "Brutalist Statue".to_string(),
        "A statue of a worker with a technical tool of time long past in a blocky, blunt style.".to_string(),
        "The plaque on the statue reads: 'Inner-Party criticism is a weapon for strengthening the Party organization and increasing its fighting capacity. In the Party organization of the Red Army, however, criticism is not always of this character, and sometimes turns into personal attack. As a result, it damages the Party organization as well as individuals. This is a manifestation of petty-bourgeois individualism. The method of correction is to help Party members understand that the purpose of criticism is to increase the Party's fighting capacity in order to achieve victory in the class struggle and that it should not be used as a means of personal attack.'".to_string(),
        LocationID::FirstTown,
    Point3D::new(0,0,0)
    );

    //this will spawn in all the NPCs and entities like statues in the first town
}
