use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
//will later read stuff like items attributes and the log
pub fn ui_render(ecs: &SubWorld, #[resource] log: &Vec<String>) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(UI_LAYER);

    draw_batch.submit(5000).expect("Batch Error");
}
//this system is gonna first grab basic information about the character like health, fatigue points, etc, and then it'll process all the new log messages and add them to the full log for the session
