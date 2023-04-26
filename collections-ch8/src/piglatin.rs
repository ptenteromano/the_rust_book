const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];

/// Useable for an entire string. Separates by whitespace
pub fn convert_str(s: &str) -> String {
    s.split_whitespace()
        .filter_map(|word| convert_word(word))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn convert_word(s: &str) -> Option<String> {
    let first_letter = s.chars().nth(0)?;

    if s.len() == 1 {
        return Some(s.to_string());
    }

    Some(match VOWELS.contains(&first_letter) {
        x if x => append_hay(s),
        _ => append_consonant_ay(s),
    })
}

fn append_hay(s: &str) -> String {
    let mut new_string = String::from(s);
    new_string.push_str("-hay");

    new_string
}

fn append_consonant_ay(s: &str) -> String {
    let mut new_string = String::from(s);
    let first_letter = new_string.remove(0);
    new_string.push_str(format!("-{}ay", first_letter).as_str());

    new_string
}

#[cfg(test)]
mod test {
    use crate::piglatin;

    #[test]
    fn convert_str_with_empty() {
        let convo = "";
        assert_eq!(piglatin::convert_str(convo), "")
    }

    #[test]
    fn convert_str_with_words() {
        let convo = "I love learning rust";
        assert_eq!(piglatin::convert_str(convo), "I ove-lay earning-lay ust-ray")
    }

    #[test]
    fn convert_word_with_empty() {
        assert_eq!(piglatin::convert_word(""), None);
    }

    #[test]
    fn convert_word_vowel() {
        assert_eq!(piglatin::convert_word("apple").unwrap(), "apple-hay");
    }

    #[test]
    fn append_hay_word() {
        assert_eq!(piglatin::append_hay("test"), "test-hay");
    }

    #[test]
    fn append_consonant_ay_word() {
        assert_eq!(piglatin::append_consonant_ay("test"), "est-tay");
    }
}
