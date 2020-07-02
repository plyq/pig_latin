// Finds character type: vowel / consonant / None

#[derive(PartialEq, Debug)]
pub enum LetterType {
    Vowel,
    Consonant,
    None,
}

pub fn letter_type(letter: char) -> LetterType {
    match &letter.to_lowercase().to_string()[..] {
        "a" | "e" | "u" | "i" | "o" => LetterType::Vowel,
        _ => {
            if letter.is_alphabetic() {
                LetterType::Consonant
            } else {
                LetterType::None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vowel() {
        assert_eq!(letter_type('A'), LetterType::Vowel);
    }

    #[test]
    fn consonant() {
        assert_eq!(letter_type('v'), LetterType::Consonant);
    }

    #[test]
    fn none() {
        assert_eq!(letter_type('_'), LetterType::None);
    }
}
