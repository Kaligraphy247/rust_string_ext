//! String extension module
//! 
//! 1. split_str

pub mod string_extention {
    //! mod string extention
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


// 2. 
pub fn capitalize(word: &str) -> String {
    //! capitalize string
    let m: &str = &word[0..1].to_uppercase();
    let n: &str = &word[1..];
    let o: String = m.to_owned() + n;
    return o
}


// 3. 
pub fn substring(word: &str, x:usize, y:usize) -> String{
    //! substring
    let result: &str = &word[x..y+1];
    return result.to_string();
}
}



// old
// fn split_string(word: &str, at: &str) -> (String, String) {
//     if word.len() != 0 {
//         let mut a: String = String::new();
//         let mut b: String = String::new();
//         a.push_str(&word[0..word.find(at).unwrap()]);
//         b.push_str(&word[(word.find(at).unwrap() + 1)..]);
//         let c = (a, b);
//         return c;
//     } 

//     else {
//         let c: (String, String) = (String::new(), String::new());
//         let error_msg = Colour::Red.bold().paint("`word` must not be empty. Please enter a string >= 2");
//         assert!(word.len() != 0, "{}", error_msg);
//         return c;
//     };
// }