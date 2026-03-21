use std::sync::OnceLock;

use regex::Regex;

fn emoji_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();

    RE.get_or_init(|| {
        Regex::new(r"[\p{Emoji_Presentation}\p{Extended_Pictographic}]")
            .expect("Invalid emoji regex")
    })
}

pub fn remove_emojis(input: &str) -> std::borrow::Cow<'_, str> {
    emoji_regex().replace_all(input, "")
}
