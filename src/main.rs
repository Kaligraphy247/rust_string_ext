
mod ext;
use crate::ext::string_extention::{split_string, capitalize, substring};



fn main() {
    // just used to test functions
    let a_string_to_split: (String, String) = split_string("example@mail.com", "@");
    let capitalize_str: String = capitalize("john doe");
    let substring_str: String = substring("example", 2, 4);
    println!("Split string: {}\nCapitalize: {}\nSubstring: {}", a_string_to_split.0, capitalize_str, substring_str)
}