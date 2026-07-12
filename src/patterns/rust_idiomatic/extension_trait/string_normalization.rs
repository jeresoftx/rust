pub trait StringNormalizationExt {
    fn normalized_spaces(&self) -> String;
    fn normalized_email(&self) -> String;
    fn slug_key(&self) -> String;
}

impl StringNormalizationExt for str {
    fn normalized_spaces(&self) -> String {
        self.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    fn normalized_email(&self) -> String {
        self.trim().to_lowercase()
    }

    fn slug_key(&self) -> String {
        let mut slug = String::new();
        let mut previous_was_separator = false;

        for character in self.trim().chars().flat_map(char::to_lowercase) {
            if character.is_ascii_alphanumeric() {
                slug.push(character);
                previous_was_separator = false;
            } else if !previous_was_separator && !slug.is_empty() {
                slug.push('-');
                previous_was_separator = true;
            }
        }

        slug.trim_end_matches('-').to_string()
    }
}
