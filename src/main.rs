use std::usize;

// use ansi_term::{Colour, Style};




mod ext;
use crate::ext::string_extention::{split_string};



fn main() {
    // println!("Hello, world!");
    let part = capitalize("this");
    let _sub  = substring("word uygygfyjv", 2, 7);

    let email = "example@mail.com";
    // let emt = "tye";
    // let red_string = Colour::Red.bold().paint("a red string").to_string();

    let c = split_string(email, "@");
    // let d = &c.0;


    

    // let aa = email.find('@');

    println!("{}\n{:?}", part , c);
}



fn capitalize(word: &str) -> String {
    let m: &str = &word[0..1].to_uppercase();
    let n: &str = &word[1..];
    // let o: &str = m.to_owned() + &n.to_owned();
    // let o = &[m, n].concat();
    let o: String = m.to_owned() + n;
    return o
}

fn substring(word: &str, x:usize, y:usize) -> String{
    let result: &str = &word[x..y+1];
    return result.to_string();
}

// fn split_string(word: &str, at: &str) -> (String, String) {
//     let mut a: String = String::new();
//     let mut b: String = String::new();

//     a.push_str(&word[0..word.find(at).unwrap()]);
//     b.push_str(&word[(word.find(at).unwrap() + 1)..]);
//     let c = (a, b);
//     return c
// }



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

// fn split_string(word: &str, at: &str) -> (String, String) {
//     let word_length: usize = word.len();

//     match  word_length {
//         0 => {
//             let error_msg = Colour::Red.bold().paint("`word` cannot be empty");
//             panic!("{}",error_msg);
//             // return (String::new(), String::new())
//         },
//         1 => {
//             let error_msg = Colour::Red.bold().paint("`word` has to be >= 2");
//             panic!("{}",error_msg);
//             // return (String::new(), String::new())
//         },
//         _other => {
//             let mut a: String = String::new();
//             let mut b: String = String::new();
//             a.push_str(&word[0..word.find(at).unwrap()]);
//             b.push_str(&word[(word.find(at).unwrap() + 1)..]);
//             let c = (a, b);
//             return c;
//             // println!("The right thing to do");
//             // return (String::new(), String::new())
//         }
//     };
// }