use collections_ch8::piglatin;

fn main() {
    // Strings - Piglatin
    let convo = "Hello my name is phil and this is what I like to say. I love the game. This is the best thing ever";
    let piglatin_result = piglatin::convert_str(convo);

    println!("{}", piglatin_result);
}
