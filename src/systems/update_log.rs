use crate::prelude::*;

pub fn update_log(state: &mut State, commands: &mut CommandBuffer) {
    //collect and add the messages sent by other systems and add them to the log.
    for (entity, message_text) in state.ecs.query_mut::<&AddToLog>() {
        state.log.push(message_text.body.clone());
        commands.despawn(entity);
    }
}
