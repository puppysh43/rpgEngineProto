use crate::prelude::*;
//TODO add commmand buffer
const DESC_WIDTH: i32 = 35;
//screen height 35
//screen width 70
//use effects layer
pub fn examine_entity(state: &mut State, commands: &mut CommandBuffer) {
    let mut description = String::new(); //variable to hold the description to be displayed
    let mut line_num: i32 = 3;
    let mut formatted_text = Vec::<String>::new();
    for (entity, long_desc) in state
        .ecs
        .query::<With<&LongDescription, &Examining>>()
        .iter()
    {
        description = long_desc.0.clone(); //grab the long description of any item tagged as being examined
        commands.remove_one::<Examining>(entity); //then remove the examining/being examined tag
    }

    //will need to format the description string so that it can fit on the screen and be pretty
    let desc_usize = usize::try_from(DESC_WIDTH).unwrap();
    while description.len() >= desc_usize {
        let (firsthalf, secondhalf) = description.split_at(desc_usize);
        formatted_text.push(firsthalf.to_string());
        description = secondhalf.to_string();
    }

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(MAIN_LAYER);
    draw_batch.cls();
    draw_batch.target(EFFECTS_LAYER);

    for line in formatted_text {
        draw_batch.print_color_centered(line_num, line, ColorPair::new(ORANGE, BLACK));
        line_num += 1;
    }

    draw_batch.submit(5000).expect("Batch Error");

    //for loop through entire vec of strings that prints centered in color

    //this section will just format the text and print it to the screen in a way that doesn't suck
    //(this is the hard part)
    //will need to send a null character to all layers to clear the screen and then send print to the "effects" screen which I'll probably use for general UI for now
}
