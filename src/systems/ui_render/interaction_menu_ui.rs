use crate::prelude::*;
use crate::systems::library::*;
use crate::systems::ui_render::library::*;

pub fn render_interaction_menu(state: &mut State) {
    match state.controlstate {
        ControlState::InteractionMenu(int_menu_state) => match int_menu_state {
            InteractionMenuState::MakingChoice => draw_interaction_menu(state),
            InteractionMenuState::ViewingResult => draw_result(state),
        },
        _ => {
            //do nothing/this shouldn't happen
        }
    }
}

fn draw_interaction_menu(state: &mut State) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(MAIN_LAYER);
    draw_batch.cls();
    draw_batch.target(EFFECTS_LAYER);
    //will need to go through and check to see which ones are visible
    let active_interactionmenu = get_active_interactionmenu(state).unwrap();
    let visible_entries = active_interactionmenu.get_visible_entries(state);
    let mut valid_entries: Vec<String> = Vec::new();
    for index in visible_entries.iter() {
        //get entry text of option and add it to the vec of valid entries
        valid_entries.push(active_interactionmenu.get_entry(*index).get_entry_text());
    }
    let mut line_num = 0;
    //need to print the header of the interaction menu
    let formatted_header = greedy_word_wrap(active_interactionmenu.get_header_text(), 60);
    for line in formatted_header {
        draw_batch.print(Point::new(10, line_num), line);
        line_num += 1;
    }
    line_num += 2;
    //then print all the visible options
    let mut option_num = 1;
    for line in valid_entries {
        let fmt_option = format!("{}. {}", option_num, line);
        draw_batch.print(Point::new(2, line_num), fmt_option);
        line_num += 2;
        option_num += 1;
    }

    draw_batch.submit(5000).expect("Batch Error");
}

fn draw_result(state: &mut State) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(MAIN_LAYER);
    draw_batch.cls();
    draw_batch.target(EFFECTS_LAYER);
    //get the active interaction menu to interface with
    let active_interactionmenu = get_active_interactionmenu(state);
    //var to hold result outside of query
    let mut interactionmenu_result: Option<InteractionMenuResult> = None;
    for (_moi_id, moi) in state.ecs.query::<&InteractionMenuResult>().iter() {
        interactionmenu_result = Some(moi.clone());
    }
    let result_text = active_interactionmenu
        .unwrap()
        .get_entry(interactionmenu_result.unwrap().current_option_index)
        .get_result_text(interactionmenu_result.unwrap().choice_result);
    let mut print_y = 2;
    for line in greedy_word_wrap(result_text, 60) {
        draw_batch.print(Point::new(2, print_y), line);
        print_y += 1;
    }
    //to draw the result we need to query the result MOI and then feed it into the active interaction menu entry
    //to get the result text that needs to be displayed on screen
    draw_batch.submit(5000).expect("Batch Error");
}
