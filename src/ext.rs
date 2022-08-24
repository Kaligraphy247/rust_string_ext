//! String extension module
//! 
//! 1. split_str

pub mod string_extention {
    //! mod 
pub fn split_string(word: &str, at: &str) -> (String, String) {
    //! Split string function
    let word_length: usize = word.len();

    match  word_length {
        0 => {
            let error_msg = ansi_term::Colour::Red.bold().paint("`word` cannot be empty");
            panic!("{}",error_msg);
        },
        1 => {
            let error_msg = ansi_term::Colour::Red.bold().paint("`word` has to be >= 2");
            panic!("{}",error_msg);
        },
        _other => {
            let mut a: String = String::new();
            let mut b: String = String::new();
            a.push_str(&word[0..word.find(at).unwrap()]);
            b.push_str(&word[(word.find(at).unwrap() + 1)..]);
            let c = (a, b);
            return c;
        }
    };
}


}