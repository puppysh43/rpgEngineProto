use crate::prelude::*;

//word wrap function
pub fn greedy_word_wrap(raw: String, width: i32) -> Vec<String> {
    let raw_split = raw.as_str().split_ascii_whitespace();
    let mut fmt_string: Vec<String> = Vec::new();
    let mut temp_string = String::new();
    for word in raw_split {
        if (temp_string.len() as i32 + word.len() as i32) <= width {
            temp_string.push(' ');
            temp_string = temp_string + word;
        } else {
            fmt_string.push(temp_string.clone());
            temp_string.clear();
            temp_string = temp_string + word;
        }
    }
    fmt_string.push(temp_string);
    fmt_string
}

//get player location
