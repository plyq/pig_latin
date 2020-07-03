pub mod letter_type;
pub mod pig_latin;

#[cfg(test)]
mod tests {
    use super::pig_latin::pig_latin;

    #[test]
    fn non_mutable_word() {
        let word = String::from("some");
        let result = pig_latin(word);
        assert_eq!(result, "ome-say");
    }

    #[test]
    fn mutable_word() {
        let mut word = String::from("some");
        let mut result = pig_latin(word);
        assert_eq!(result, "ome-say");
        word = String::from("any");
        assert_eq!(result, "ome-say");
        result = pig_latin(word);
        assert_eq!(result, "any-hay");
    }
}
