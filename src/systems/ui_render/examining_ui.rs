use crate::prelude::*;
//TODO add commmand buffer
pub fn examine_entity(state: &mut State) {
    let mut description = String::new(); //variable to hold the description to be displayed
    for (entity, long_desc) in state
        .ecs
        .query::<With<&LongDescription, &Examining>>()
        .iter()
    {
        description = long_desc.0.clone(); //grab the long description of any item tagged as being examined
                                           //then remove the examining/being examined tag from the entity in the for loop using a command buffer
    }
    //this section will just format the text and print it to the screen in a way that doesn't suck
    //(this is the hard part)
    //will need to send a null character to all layers to clear the screen and then send print to the "effects" screen which I'll probably use for general UI for now
}
