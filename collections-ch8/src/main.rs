pub mod company;
pub mod piglatin;
pub mod vectors;

/// The Book Ch. 8 - Common Collections
/// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
/// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
/// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

fn main() {
    // Vectors - Median and Mode
    let vec = vec![9, 5, 2, 9, 4, 3, 2, 2, 1];

    let median = vectors::find_median(&vec);
    match median {
        Some(x) => println!("Found median: {}", x),
        _ => println!("No median found!"),
    }

    let mode = vectors::find_mode(&vec);
    match mode {
        Some(x) => println!("Found mode: {}", x),
        _ => println!("No mode found!"),
    }

    // Strings - Piglatin
    let convo = "Hello my name is phil and this is what I like to say. I love the game. This is the best thing ever";
    let piglatin_result = piglatin::convert_str(convo);

    println!("{}", piglatin_result);

    // Hashmap - Company organization
}
