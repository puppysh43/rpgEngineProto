use crate::prelude::*;
///This is the root level data structure that contains all Interaction Menus for
///every entity in yr game. To play nice with the borrowchecker it will never
///give you direct access to them after being constructed and will only give you
///a clone of a specific interaction menu
pub struct InteractionMenuDatabase {
    contents: HashMap<String, InteractionMenu>,
}
impl InteractionMenuDatabase {
    pub fn new(contents: HashMap<String, InteractionMenu>) -> Self {
        Self { contents }
    }
    pub fn get_interaction_menu(&self, key: String) -> Option<InteractionMenu> {
        if self.contents.contains_key(&key) {
            let int_menu = self.contents.get(&key).unwrap().clone();
            Some(int_menu)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
///A single tiered interaction menu for basic player interactions with the environment
///that isn't dialogue. Ex. a boulder blocking a doorway that can be pushed w/ a str
///check, destroyed with an explosive item, or moved if you have NPC help.
pub struct InteractionMenu {
    ///The header contains the text describing the in-game situation the interaction menu is representing
    header: String,
    pub choices: Vec<IntMenuChoice>,
}
impl InteractionMenu {
    pub fn new(header: String, choices: Vec<IntMenuChoice>) -> Self {
        InteractionMenu { header, choices }
    }
    pub fn new_blank(header: String) -> Self {
        InteractionMenu {
            header,
            choices: Vec::new(),
        }
    }
    pub fn add_entry(&mut self, entry: IntMenuChoice) {
        self.choices.push(entry);
    }
    pub fn get_entry(&self, index: usize) -> IntMenuChoice {
        self.choices[index].clone()
    }
    ///This method returns a vector of all the indexes of entries visible to the player
    ///this vector can easily be iterated over for either interpreting input when
    ///selecting an option or finding what to display in yr UI interface of choice
    pub fn get_visible_entries(&self, state: &State) -> Vec<usize> {
        let mut vis_entries: Vec<usize> = Vec::new();
        let mut entries_index: usize = 0;
        //iterate through all entries in the current menu
        for entry in self.choices.iter() {
            //check each entry in the menu if the visibility conditions are met
            if entry.check_visibility_condition(state) == true {
                //if they are add them to the vec of usizes
                vis_entries.push(entries_index)
            }
            entries_index += 1; //increment the index to track where in the vec we are
        }
        //return the vector of the indexes of all the menu entries that can be shown
        vis_entries
    }
    ///Get the header of the interaction menu for printing purposes
    pub fn get_header_text(&self) -> String {
        self.header.clone()
    }
}
///This is the base datatype for every node. It contains the text shown when displaying
///all options in the interaction menu, optional visibility conditions when at the root
///of the menu, optional checks and consequences, and the different strings that will
///be displayed depending on the result of aformentioned checks and consequences.
#[derive(Clone, Debug)]
pub struct IntMenuChoice {
    ///The text displayed on the root level of the interaction menu describing what the
    ///option actually is (ex. "[Strength Check] Push The Boulder")
    entry_text: String,
    ///Each entry has the option of conditional visibility, meaning it will only be displayed if
    ///certain conditions are met. This can prevent metagaming/be more "realistic", preventing
    ///players from seeing that they can use items they're supposed to find organically or
    ///options only available after a related quest has been completed.
    vis_condition: Option<VisCondition>,
    ///Each entry has the option of being able to read and write to the gamestate. This is
    ///intended to allow the engine to implement a wide variety of RPG mechanics - skillchecks,
    ///faction reputation gating, quest scripting, etc. This can be any function that read/writes
    ///to the gamestate as long as it produces a choice result that can be parsed. Go nuts!
    c_and_c: Option<ChecksAndConsequences>,
    ///Each entry will have at least one piece of text that displayed after that option has been chosen
    result_text: ResultText, //vec of different things that can be printed to the screen as a result, indexed by casting the result into a usize for accessing the vec of strings
}
impl IntMenuChoice {
    pub fn new(
        entry_text: String,
        vis_condition: Option<VisCondition>,
        c_and_c: Option<ChecksAndConsequences>,
        result_text: ResultText,
    ) -> Self {
        Self {
            entry_text,
            vis_condition,
            c_and_c,
            result_text,
        }
    }
    ///Method that checks the gamestate against the visibility conditions of
    ///the interaction menu choice and returns a boolean
    pub fn check_visibility_condition(&self, state: &State) -> bool {
        //an interaction menu entry can
        if self.vis_condition.is_some() {
            //if there's a visibility condition we can safely unwrap it and run it on the state
            //this function was given.
            let vis_check = self.vis_condition.unwrap();
            vis_check(state)
        } else {
            //if there's no visibility conditions then the option is always
            //supposed to be visible so just return true.
            true
        }
    }
    ///returns the header text of the interaction menu entry as a string slice for the purpose of printing
    pub fn get_entry_text(&self) -> String {
        self.entry_text.clone()
    }
    ///returns the result text for the purpose of printing or otherwise processing
    pub fn get_result_text(&self, choice: Option<ChoiceResult>) -> String {
        //testing if choice is some
        if choice.is_some() {
            //actually access the contents
            match choice.unwrap() {
                ChoiceResult::BinaryResult(result) => match result {
                    false => self.result_text.options[0].clone(),
                    true => self.result_text.options[1].clone(),
                },
                ChoiceResult::DegOfSuccess(result) => match result {
                    DegreeOfSuccess::Failure => self.result_text.options[0].clone(),
                    DegreeOfSuccess::PartialSuccess => self.result_text.options[1].clone(),
                    DegreeOfSuccess::FullSuccess => self.result_text.options[2].clone(),
                },
            }
        } else {
            //if there's no result it's b/c there's only one way it'll go so just print the one result
            self.result_text.options[0].clone()
        }
    }
    ///Method that lets the checks and consequences logic of the interaction menu choice
    ///have read-write access to the gamestate.
    pub fn run_checks_and_consequences(
        &self,
        state: &mut State,
        commands: &mut CommandBuffer,
    ) -> Option<ChoiceResult> {
        //if there are checks and consequences unwrap and run it.
        if self.c_and_c.is_some() {
            let checks_and_consequences = self.c_and_c.unwrap();
            let result = checks_and_consequences(state, commands);
            result
        } else {
            None
        }
    }
}
///Datatype used for checking if an interaction menu choice should be displayed
///it should only ever need read-only access (non-mutable reference) to the gamestate
///and based on that will return a bool representing if it can be shown or not.
type VisCondition = fn(&State) -> bool;
///Type used for
type ChecksAndConsequences = fn(&mut State, &mut CommandBuffer) -> Option<ChoiceResult>;

#[derive(Clone, Debug, PartialEq)]
///This datatype contains the different possible results of an option in an
///interaction menu. When an entry has been chosen and checks have been run
///this is what will be referenced in order to see what will need to be printed
///but this doesn't contain any logic itself.
pub struct ResultText {
    options: Vec<String>,
}
impl ResultText {
    ///This method makes the result text for an interaction menu entry
    ///that doesn't have any result such as examining an object or
    ///pulling a lever
    pub fn new_static_result_text(result: String) -> Self {
        Self {
            options: vec![result],
        }
    }
    ///This method makes the result text for an interaction menu entry that
    ///has a binary result, such as whether or not the player has a key for a
    ///lock or has completed a requisite quest.
    pub fn new_binary_result_text(false_result: String, true_result: String) -> Self {
        Self {
            options: vec![false_result, true_result],
        }
    }
    ///This method makes the result text for an interaction menu entry that
    ///has degrees of success as a result, almost always this will be in the
    ///case of a skillcheck.
    pub fn new_deg_of_success_result_text(
        failure: String,
        partial_success: String,
        full_success: String,
    ) -> Self {
        Self {
            options: vec![failure, partial_success, full_success],
        }
    }
}
