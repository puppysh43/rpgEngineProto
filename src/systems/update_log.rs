use crate::prelude::*;
#[system]
#[read_component(AddToLog)]
pub fn update_log(ecs: &SubWorld, commands: &mut CommandBuffer, #[resource] log: &mut Vec<String>) {
    //collect and add the messages sent by other systems and add them to the log.
    let mut new_messages = <&AddToLog>::query();
    new_messages.iter(ecs).for_each(|message| {
        log.push(message.body.clone());
    });
    //collect and remove the messages you've just processed and added to the log
    // let mut old_messages = <&Entity>::query().filter(component::<AddToLog>());
    // old_messages.iter(ecs).for_each(|entity| {
    // commands.remove(*entity);
    // });
    //need to find a way to remove all the messages to avoid memory leak
}
