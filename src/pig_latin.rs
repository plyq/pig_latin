// Transforms words to pig latin

use super::letter_type::{letter_type, LetterType};

pub fn pig_latin(word: String) -> String {
    // Transforms word to pig latin word
    let mut char_with_index = word.char_indices();

    match char_with_index.next() {
        Some((pos, first_letter)) => {
            let word_end = &word[pos + 1..];
            match letter_type(first_letter) {
                LetterType::Vowel => format!("{}-hay", &word),
                LetterType::Consonant => format!("{}-{}ay", word_end, first_letter),
                LetterType::Other => word,
            }
        }
        None => word,
    }
}

#[cfg(test)]
mod tests {
    use super::pig_latin;

    #[test]
    fn vowel_start() {
        let word = String::from("abc");
        assert_eq!(pig_latin(word), "abc-hay");
    }

    #[test]
    fn consonant_start() {
        let word = String::from("zxc");
        assert_eq!(pig_latin(word), "xc-zay");
    }

    #[test]
    fn digit_start() {
        let word = String::from("123");
        assert_eq!(pig_latin(word), "123");
    }

    #[test]
    fn one_vowel() {
        let word = String::from("a");
        assert_eq!(pig_latin(word), "a-hay");
    }

    #[test]
    fn one_consonant() {
        let word = String::from("b");
        assert_eq!(pig_latin(word), "-bay");
    }

    #[test]
    fn empty() {
        let word = String::new();
        assert_eq!(pig_latin(word), "");
    }
}
