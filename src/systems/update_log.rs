use crate::prelude::*;
#[system]
#[read_component(AddToLog)]
pub fn update_log(ecs: &SubWorld, commands: &mut CommandBuffer, #[resource] log: &mut Vec<String>) {
    let mut new_messages = <&AddToLog>::query();
    new_messages.iter(ecs).for_each(|message| {
        log.push(message.body.clone());
    });
    //need to find a way to remove all the messages to avoid memory leak
}
